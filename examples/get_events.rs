//! Example: Testing the get_events method of the DFlow Prediction Market API client
//!
//! Run with:
//! ```bash
//! DFLOW_API_KEY=your-api-key cargo run --example get_events
//! ```

use dflow_api_client::prediction::{
    DflowPredictionApiClient, GetEventsParams, MarketStatus, SortField,
};

#[tokio::main]
async fn main() {
    // Get API key from environment variable
    let api_key = std::env::var("DFLOW_API_KEY").unwrap_or_else(|_| {
        eprintln!("Warning: DFLOW_API_KEY not set, using empty string");
        String::new()
    });

    // Create client with default URL
    let client = DflowPredictionApiClient::new(
        "https://dev-prediction-markets-api.dflow.net".to_string(),
        api_key,
    );

    println!("=== DFlow Prediction Market API - get_events Example ===\n");

    // Example 1: Get events with no params (default behavior)
    println!("1. Fetching events with default params...");
    match client.get_events(None).await {
        Ok(response) => {
            println!("   Found {} events", response.events.len());
            if let Some(cursor) = response.cursor {
                println!("   Next cursor: {}", cursor);
            }
            for event in response.events.iter().take(3) {
                println!("   - {} | {}", event.ticker, event.title);
            }
            if response.events.len() > 3 {
                println!("   ... and {} more", response.events.len() - 3);
            }
        }
        Err(e) => eprintln!("   Error: {e:#?}"),
    }
    println!();

    println!("\n=== Done ===");
}
