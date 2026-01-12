//! DFlow API Client
//!
//! A Rust client library for interacting with the DFlow API.
//!
//! ## Features
//!
//! - **Prediction Market Metadata API**: Query and retrieve prediction market information,
//!   events, markets, candlestick data, and more.
//! - **Swap API**: Execute token swaps via imperative or declarative (intent-based) flows.
//!
//! ## Example - Prediction Markets
//!
//! ```no_run
//! use dflow_api_client::prediction::{
//!     DflowPredictionApiClient, GetEventsParams, MarketStatus,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a client with your API key
//!     let client = DflowPredictionApiClient::with_default_url(
//!         "your-api-key".to_string(),
//!     );
//!
//!     // Get all active events
//!     let params = GetEventsParams {
//!         status: Some(MarketStatus::Active),
//!         limit: Some(10),
//!         ..Default::default()
//!     };
//!
//!     let events = client.get_events(Some(params)).await.unwrap();
//!     for event in events.events {
//!         println!("Event: {} - {}", event.ticker, event.title);
//!     }
//! }
//! ```
//!
//! ## Example - Swap API
//!
//! ```no_run
//! use dflow_api_client::swap::{DflowSwapApiClient, GetQuoteParams};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a swap client with your API key
//!     let client = DflowSwapApiClient::with_default_url(
//!         "your-api-key".to_string(),
//!     );
//!
//!     // Get a quote for swapping SOL to USDC
//!     let params = GetQuoteParams {
//!         input_mint: "So11111111111111111111111111111111111111112".to_string(),
//!         output_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
//!         amount: "1000000000".to_string(), // 1 SOL
//!         slippage_bps: Some(50), // 0.5%
//!         ..Default::default()
//!     };
//!
//!     let quote = client.get_quote(params).await.unwrap();
//!     println!("Output amount: {}", quote.out_amount);
//! }
//! ```

pub mod prediction;
pub mod swap;

// Re-export common types at the crate level for convenience
pub use prediction::{
    DEFAULT_BASE_URL as PREDICTION_DEFAULT_BASE_URL, DflowPredictionApiClient,
    DflowPredictionApiError, Result as PredictionResult,
};
pub use swap::{
    DEFAULT_BASE_URL as SWAP_DEFAULT_BASE_URL, DflowSwapApiClient,
    DflowSwapApiError, Result as SwapResult,
};
