//! WebSocket client for the DFlow Prediction Market API.
//!
//! This module provides real-time streaming of market data via WebSocket,
//! including price updates, trade executions, and orderbook depth.
//!
//! # Example
//!
//! ```no_run
//! use dflow_api_client::prediction::websocket::{
//!     Channel, DflowPredictionWsClient,
//! };
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to the WebSocket
//!     let client = DflowPredictionWsClient::connect().await?;
//!
//!     // Subscribe to all price updates
//!     let (mut stream, unsubscribe) = client.prices_subscribe_all().await?;
//!
//!     // Process incoming price updates
//!     while let Some(update) = stream.next().await {
//!         println!("Price update: {:?}", update);
//!     }
//!
//!     // Cleanup
//!     unsubscribe().await;
//!     client.shutdown().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod types;

use std::collections::BTreeMap;

use futures_util::{
    SinkExt,
    future::BoxFuture,
    stream::{BoxStream, StreamExt},
};
use serde_json::Value;
use thiserror::Error;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::{Duration, sleep},
};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{
        Message,
        http::Request,
        protocol::frame::{CloseFrame, coding::CloseCode},
    },
};
pub use types::*;

/// Default WebSocket URL for the DFlow Prediction Market API
pub const DEFAULT_WS_URL: &str =
    "wss://prediction-markets-api.dflow.net/api/v1/ws";

/// Default ping interval in seconds
pub const DEFAULT_PING_INTERVAL_SECS: u64 = 30;

// =============================================================================
// Error Types
// =============================================================================

/// Errors that can occur when using the WebSocket client.
#[derive(Debug, Error)]
pub enum DflowWsError {
    /// WebSocket connection failed
    #[error("WebSocket connection failed: {0}")]
    ConnectionFailed(#[from] tokio_tungstenite::tungstenite::Error),

    /// WebSocket connection was closed
    #[error("WebSocket connection closed: {0}")]
    ConnectionClosed(String),

    /// Failed to serialize message
    #[error("Failed to serialize message: {0}")]
    SerializeError(#[from] serde_json::Error),

    /// Failed to send message (channel closed)
    #[error("Failed to send message: channel closed")]
    SendFailed,

    /// Subscription failed
    #[error("Subscription failed: {0}")]
    SubscriptionFailed(String),
}

/// Result type for WebSocket operations.
pub type WsResult<T> = Result<T, DflowWsError>;

// =============================================================================
// Internal Types
// =============================================================================

type UnsubscribeFn = Box<dyn FnOnce() -> BoxFuture<'static, ()> + Send>;
type SubscribeResponseMsg =
    WsResult<(mpsc::UnboundedReceiver<Value>, UnsubscribeFn)>;
type SubscribeRequestMsg =
    (SubscribeMessage, oneshot::Sender<SubscribeResponseMsg>);
type SubscribeResult<'a, T> = WsResult<(BoxStream<'a, T>, UnsubscribeFn)>;

// =============================================================================
// WebSocket Client
// =============================================================================

/// A WebSocket client for streaming real-time data from the DFlow Prediction Market API.
///
/// This client provides methods to subscribe to various channels (prices, trades, orderbook)
/// and receive updates as async streams.
///
/// # Example
///
/// ```no_run
/// use dflow_api_client::prediction::websocket::DflowPredictionWsClient;
/// use futures_util::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = DflowPredictionWsClient::connect().await?;
///
///     let (mut prices, _unsub) = client.prices_subscribe_all().await?;
///     while let Some(price) = prices.next().await {
///         println!("{:?}", price);
///     }
///
///     Ok(())
/// }
/// ```
pub struct DflowPredictionWsClient {
    subscribe_sender: mpsc::UnboundedSender<SubscribeRequestMsg>,
    shutdown_sender: Option<oneshot::Sender<()>>,
    ws_task: Option<JoinHandle<WsResult<()>>>,
}

impl DflowPredictionWsClient {
    /// Connect to the DFlow WebSocket API using the default URL.
    ///
    /// # Returns
    ///
    /// A connected `DflowPredictionWsClient` ready for subscriptions.
    ///
    /// # Errors
    ///
    /// Returns an error if the WebSocket connection fails.
    pub async fn connect() -> WsResult<Self> {
        Self::connect_with_url(DEFAULT_WS_URL).await
    }

    /// Connect to the DFlow WebSocket API using a custom URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket URL to connect to
    ///
    /// # Returns
    ///
    /// A connected `DflowPredictionWsClient` ready for subscriptions.
    pub async fn connect_with_url(url: &str) -> WsResult<Self> {
        Self::connect_with_url_and_headers(url, &[]).await
    }

    /// Connect to the DFlow WebSocket API using an API key for authentication.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authentication
    ///
    /// # Returns
    ///
    /// A connected `DflowPredictionWsClient` ready for subscriptions.
    pub async fn connect_with_api_key(api_key: &str) -> WsResult<Self> {
        Self::connect_with_url_and_headers(
            DEFAULT_WS_URL,
            &[("Authorization", &format!("Bearer {}", api_key))],
        )
        .await
    }

    /// Connect to the DFlow WebSocket API using a custom URL and headers.
    ///
    /// # Arguments
    ///
    /// * `url` - The WebSocket URL to connect to
    /// * `headers` - A slice of header key-value pairs to include in the connection request
    ///
    /// # Returns
    ///
    /// A connected `DflowPredictionWsClient` ready for subscriptions.
    pub async fn connect_with_url_and_headers(
        url: &str,
        headers: &[(&str, &str)],
    ) -> WsResult<Self> {
        let mut request = Request::builder()
            .uri(url)
            .header("Host", url_host(url).unwrap_or_default())
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tokio_tungstenite::tungstenite::handshake::client::generate_key(
                ),
            );

        for (key, value) in headers {
            request = request.header(*key, *value);
        }

        let request = request
            .body(())
            .map_err(|e| DflowWsError::ConnectionClosed(e.to_string()))?;

        let (ws, _response) = connect_async(request).await?;

        let (subscribe_sender, subscribe_receiver) = mpsc::unbounded_channel();
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();

        let ws_task = tokio::spawn(Self::run_ws(
            ws,
            subscribe_receiver,
            shutdown_receiver,
            DEFAULT_PING_INTERVAL_SECS,
        ));

        Ok(Self {
            subscribe_sender,
            shutdown_sender: Some(shutdown_sender),
            ws_task: Some(ws_task),
        })
    }

    /// Gracefully shutdown the WebSocket connection.
    ///
    /// This will close the connection and wait for the background task to complete.
    pub async fn shutdown(mut self) -> WsResult<()> {
        // Send shutdown signal
        if let Some(sender) = self.shutdown_sender.take() {
            let _ = sender.send(());
        }

        // Wait for the WebSocket task to complete
        if let Some(ws_task) = self.ws_task.take() {
            ws_task.await.map_err(|_| {
                DflowWsError::ConnectionClosed(
                    "WebSocket task panicked".to_string(),
                )
            })??;
        }

        Ok(())
    }

    // =========================================================================
    // Prices Channel
    // =========================================================================

    /// Subscribe to price updates for all markets.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A stream of `PriceUpdate` messages
    /// - An unsubscribe function to stop receiving updates
    pub async fn prices_subscribe_all(
        &self,
    ) -> SubscribeResult<'_, PriceUpdate> {
        self.subscribe_channel(SubscribeMessage::all(Channel::Prices))
            .await
    }

    /// Subscribe to price updates for specific market tickers.
    ///
    /// # Arguments
    ///
    /// * `tickers` - List of market ticker IDs to subscribe to
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A stream of `PriceUpdate` messages
    /// - An unsubscribe function to stop receiving updates
    pub async fn prices_subscribe_tickers(
        &self,
        tickers: Vec<String>,
    ) -> SubscribeResult<'_, PriceUpdate> {
        self.subscribe_channel(SubscribeMessage::tickers(
            Channel::Prices,
            tickers,
        ))
        .await
    }

    // =========================================================================
    // Trades Channel
    // =========================================================================

    /// Subscribe to trade updates for all markets.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A stream of `TradeUpdate` messages
    /// - An unsubscribe function to stop receiving updates
    pub async fn trades_subscribe_all(
        &self,
    ) -> SubscribeResult<'_, TradeUpdate> {
        self.subscribe_channel(SubscribeMessage::all(Channel::Trades))
            .await
    }

    /// Subscribe to trade updates for specific market tickers.
    ///
    /// # Arguments
    ///
    /// * `tickers` - List of market ticker IDs to subscribe to
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A stream of `TradeUpdate` messages
    /// - An unsubscribe function to stop receiving updates
    pub async fn trades_subscribe_tickers(
        &self,
        tickers: Vec<String>,
    ) -> SubscribeResult<'_, TradeUpdate> {
        self.subscribe_channel(SubscribeMessage::tickers(
            Channel::Trades,
            tickers,
        ))
        .await
    }

    // =========================================================================
    // Orderbook Channel
    // =========================================================================

    /// Subscribe to orderbook updates for all markets.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A stream of `OrderbookUpdate` messages
    /// - An unsubscribe function to stop receiving updates
    pub async fn orderbook_subscribe_all(
        &self,
    ) -> SubscribeResult<'_, OrderbookUpdate> {
        self.subscribe_channel(SubscribeMessage::all(Channel::Orderbook))
            .await
    }

    /// Subscribe to orderbook updates for specific market tickers.
    ///
    /// # Arguments
    ///
    /// * `tickers` - List of market ticker IDs to subscribe to
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - A stream of `OrderbookUpdate` messages
    /// - An unsubscribe function to stop receiving updates
    pub async fn orderbook_subscribe_tickers(
        &self,
        tickers: Vec<String>,
    ) -> SubscribeResult<'_, OrderbookUpdate> {
        self.subscribe_channel(SubscribeMessage::tickers(
            Channel::Orderbook,
            tickers,
        ))
        .await
    }

    // =========================================================================
    // Internal Methods
    // =========================================================================

    /// Internal method to subscribe to a channel and return a typed stream.
    async fn subscribe_channel<'a, T>(
        &self,
        msg: SubscribeMessage,
    ) -> SubscribeResult<'a, T>
    where
        T: serde::de::DeserializeOwned + Send + 'a,
    {
        let (response_sender, response_receiver) = oneshot::channel();

        self.subscribe_sender
            .send((msg, response_sender))
            .map_err(|_| DflowWsError::SendFailed)?;

        let (notifications, unsubscribe) =
            response_receiver.await.map_err(|_| {
                DflowWsError::ConnectionClosed(
                    "Response channel closed".to_string(),
                )
            })??;

        let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(notifications)
            .filter_map(|value| async move {
                match serde_json::from_value::<T>(value.clone()) {
                    Ok(parsed) => Some(parsed),
                    Err(e) => {
                        eprintln!(
                            "Failed to parse WebSocket message: {:?} for value: {:?}",
                            e, value
                        );
                        None
                    }
                }
            })
            .boxed();

        Ok((stream, unsubscribe))
    }

    /// Background task that manages the WebSocket connection.
    async fn run_ws(
        mut ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
        mut subscribe_receiver: mpsc::UnboundedReceiver<SubscribeRequestMsg>,
        mut shutdown_receiver: oneshot::Receiver<()>,
        ping_interval_secs: u64,
    ) -> WsResult<()> {
        // Track subscriptions by channel
        // Key: channel name, Value: sender for notifications
        let mut subscriptions: BTreeMap<String, mpsc::UnboundedSender<Value>> =
            BTreeMap::new();
        let (unsubscribe_sender, mut unsubscribe_receiver) =
            mpsc::unbounded_channel::<(Channel, oneshot::Sender<()>)>();

        loop {
            tokio::select! {
                // Handle shutdown signal
                _ = &mut shutdown_receiver => {
                    let frame = CloseFrame {
                        code: CloseCode::Normal,
                        reason: "Client shutdown".into(),
                    };
                    let _ = ws.send(Message::Close(Some(frame))).await;
                    let _ = ws.flush().await;
                    break;
                }

                // Send periodic ping to keep connection alive
                _ = sleep(Duration::from_secs(ping_interval_secs)) => {
                    if let Err(e) = ws.send(Message::Ping(vec![])).await {
                        eprintln!("Failed to send ping: {:?}", e);
                        break;
                    }
                }

                // Handle subscription requests
                Some((subscribe_msg, response_sender)) = subscribe_receiver.recv() => {
                    let channel = subscribe_msg.channel;
                    let channel_name = channel.as_str().to_string();

                    // Serialize and send the subscription message
                    let msg_json = match serde_json::to_string(&subscribe_msg) {
                        Ok(json) => json,
                        Err(e) => {
                            let _ = response_sender.send(Err(DflowWsError::SerializeError(e)));
                            continue;
                        }
                    };

                    if let Err(e) = ws.send(Message::Text(msg_json)).await {
                        let _ = response_sender.send(Err(DflowWsError::ConnectionFailed(e)));
                        continue;
                    }

                    // Create notification channel for this subscription
                    let (notifications_sender, notifications_receiver) = mpsc::unbounded_channel();

                    // Store the sender for routing messages
                    subscriptions.insert(channel_name.clone(), notifications_sender);

                    // Create unsubscribe function
                    let unsub_sender = unsubscribe_sender.clone();
                    let unsubscribe: UnsubscribeFn = Box::new(move || {
                        Box::pin(async move {
                            let (response_sender, response_receiver) = oneshot::channel();
                            if unsub_sender.send((channel, response_sender)).is_ok() {
                                let _ = response_receiver.await;
                            }
                        })
                    });

                    let _ = response_sender.send(Ok((notifications_receiver, unsubscribe)));
                }

                // Handle unsubscribe requests
                Some((channel, response_sender)) = unsubscribe_receiver.recv() => {
                    let channel_name = channel.as_str().to_string();
                    subscriptions.remove(&channel_name);

                    // Send unsubscribe message to server
                    let unsub_msg = SubscribeMessage::unsubscribe_all(channel);
                    if let Ok(msg_json) = serde_json::to_string(&unsub_msg) {
                        let _ = ws.send(Message::Text(msg_json)).await;
                    }

                    let _ = response_sender.send(());
                }

                // Handle incoming WebSocket messages
                next_msg = ws.next() => {
                    let msg = match next_msg {
                        Some(Ok(msg)) => msg,
                        Some(Err(e)) => {
                            eprintln!("WebSocket error: {:?}", e);
                            break;
                        }
                        None => break,
                    };

                    match msg {
                        Message::Text(text) => {
                            // Parse to determine channel
                            if let Ok(raw) = serde_json::from_str::<RawMessage>(&text) {
                                if let Some(sender) = subscriptions.get(&raw.channel) {
                                    if let Ok(value) = serde_json::from_str::<Value>(&text) {
                                        let _ = sender.send(value);
                                    }
                                }
                            }
                        }
                        Message::Ping(data) => {
                            let _ = ws.send(Message::Pong(data)).await;
                        }
                        Message::Pong(_) => {
                            // Connection is alive
                        }
                        Message::Close(_) => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}

impl Drop for DflowPredictionWsClient {
    fn drop(&mut self) {
        // Attempt to trigger shutdown if not already done
        if let Some(sender) = self.shutdown_sender.take() {
            let _ = sender.send(());
        }
    }
}

/// Extract the host from a URL string.
fn url_host(url: &str) -> Option<&str> {
    let without_scheme = url
        .strip_prefix("wss://")
        .or_else(|| url.strip_prefix("ws://"))
        .or_else(|| url.strip_prefix("https://"))
        .or_else(|| url.strip_prefix("http://"))?;

    without_scheme.split('/').next()
}
