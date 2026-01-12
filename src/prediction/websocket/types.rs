//! WebSocket message types for the DFlow Prediction Market API.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// =============================================================================
// Channel Types
// =============================================================================

/// Available WebSocket channels for subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    /// Real-time bid/ask price updates
    Prices,
    /// Real-time trade execution updates
    Trades,
    /// Real-time orderbook depth updates
    Orderbook,
}

impl Channel {
    /// Returns the channel name as a string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            Channel::Prices => "prices",
            Channel::Trades => "trades",
            Channel::Orderbook => "orderbook",
        }
    }
}

// =============================================================================
// Subscription Message Types (Client -> Server)
// =============================================================================

/// Message type for subscribe/unsubscribe operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Subscribe,
    Unsubscribe,
}

/// Subscription request message sent to the WebSocket server.
///
/// Use this to subscribe to all markets or specific tickers on a channel.
#[derive(Debug, Clone, Serialize)]
pub struct SubscribeMessage {
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    pub channel: Channel,
    /// If true, subscribe to all markets on this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// Specific market tickers to subscribe to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<Vec<String>>,
}

impl SubscribeMessage {
    /// Create a subscription message for all markets on a channel.
    pub fn all(channel: Channel) -> Self {
        Self {
            msg_type: MessageType::Subscribe,
            channel,
            all: Some(true),
            tickers: None,
        }
    }

    /// Create a subscription message for specific tickers on a channel.
    pub fn tickers(channel: Channel, tickers: Vec<String>) -> Self {
        Self {
            msg_type: MessageType::Subscribe,
            channel,
            all: None,
            tickers: Some(tickers),
        }
    }

    /// Create an unsubscription message for all markets on a channel.
    pub fn unsubscribe_all(channel: Channel) -> Self {
        Self {
            msg_type: MessageType::Unsubscribe,
            channel,
            all: Some(true),
            tickers: None,
        }
    }

    /// Create an unsubscription message for specific tickers on a channel.
    pub fn unsubscribe_tickers(channel: Channel, tickers: Vec<String>) -> Self {
        Self {
            msg_type: MessageType::Unsubscribe,
            channel,
            all: None,
            tickers: Some(tickers),
        }
    }
}

// =============================================================================
// Response Message Types (Server -> Client)
// =============================================================================

/// Price update message from the prices channel.
///
/// Contains real-time bid/ask prices for YES and NO outcomes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    /// Always "prices"
    pub channel: String,
    /// Message type (e.g., "ticker")
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Market ticker identifier
    pub market_ticker: String,
    /// Best bid price for YES outcome (may be null)
    #[serde(default)]
    pub yes_bid: Option<String>,
    /// Best ask price for YES outcome (may be null)
    #[serde(default)]
    pub yes_ask: Option<String>,
    /// Best bid price for NO outcome (may be null)
    #[serde(default)]
    pub no_bid: Option<String>,
    /// Best ask price for NO outcome (may be null)
    #[serde(default)]
    pub no_ask: Option<String>,
}

/// Trade update message from the trades channel.
///
/// Contains information about a single trade execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeUpdate {
    /// Always "trades"
    pub channel: String,
    /// Always "trade"
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Market ticker identifier
    pub market_ticker: String,
    /// Unique trade identifier
    pub trade_id: String,
    /// Trade execution price (1-99)
    pub price: i64,
    /// Number of contracts traded
    pub count: i64,
    /// YES outcome price at execution (1-99)
    pub yes_price: i64,
    /// NO outcome price at execution (1-99)
    pub no_price: i64,
    /// YES price formatted in dollars
    pub yes_price_dollars: String,
    /// NO price formatted in dollars
    pub no_price_dollars: String,
    /// Side of the taker ("yes" or "no")
    pub taker_side: String,
    /// Trade creation time (Unix timestamp in milliseconds)
    pub created_time: i64,
}

/// Orderbook update message from the orderbook channel.
///
/// Contains the current orderbook depth for a market.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookUpdate {
    /// Always "orderbook"
    pub channel: String,
    /// Message type (e.g., "orderbook")
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Market ticker identifier
    pub market_ticker: String,
    /// Map of price (string) to quantity for YES outcome bids
    #[serde(default)]
    pub yes_bids: HashMap<String, i64>,
    /// Map of price (string) to quantity for NO outcome bids
    #[serde(default)]
    pub no_bids: HashMap<String, i64>,
}

/// A unified WebSocket message that can be any of the channel-specific updates.
#[derive(Debug, Clone)]
pub enum WsMessage {
    /// Price update from the prices channel
    Price(PriceUpdate),
    /// Trade update from the trades channel
    Trade(TradeUpdate),
    /// Orderbook update from the orderbook channel
    Orderbook(OrderbookUpdate),
}

/// Internal struct for deserializing incoming messages to determine channel.
#[derive(Debug, Deserialize)]
pub(crate) struct RawMessage {
    pub channel: String,
}
