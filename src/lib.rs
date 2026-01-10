//! DFlow API Client
//!
//! A Rust client library for interacting with the DFlow API.
//!
//! ## Features
//!
//! - **Prediction Market Metadata API**: Query and retrieve prediction market information,
//!   events, markets, candlestick data, and more.
//!
//! ## Example
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

pub mod prediction;

// Re-export common types at the crate level for convenience
pub use prediction::{
    DEFAULT_BASE_URL, DflowPredictionApiClient, DflowPredictionApiError,
    Result as PredictionResult,
};
