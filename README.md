# DFlow API Client

A Rust client library for interacting with the [DFlow Prediction Market Metadata API](https://pond.dflow.net/prediction-market-metadata-api-reference/introduction).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dflow-api-client = { git = "https://github.com/sarmatdev/dflow-api-client" }
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use dflow_api_client::prediction::{DflowPredictionApiClient, GetEventsParams, MarketStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with your API key
    let client = DflowPredictionApiClient::with_default_url("your-api-key".to_string());

    // Get active events
    let events = client
        .get_events(Some(GetEventsParams {
            status: Some(MarketStatus::Active),
            limit: Some(10),
            ..Default::default()
        }))
        .await?;

    for event in events.events {
        println!("{}: {}", event.ticker, event.title);
    }

    Ok(())
}
```

## API Coverage

### Events API

- `get_event` - Get a single event by ticker
- `get_events` - Get paginated list of events
- `get_event_forecast_percentile_history` - Get forecast percentile history
- `get_event_forecast_percentile_history_by_mint` - Get forecast history by mint
- `get_event_candlesticks` - Get OHLC candlestick data

### Markets API

- `get_market` - Get a single market by ticker
- `get_market_by_mint` - Get market by mint address
- `get_markets` - Get paginated list of markets
- `get_markets_batch` - Batch fetch markets by tickers
- `get_outcome_mints` - Get all outcome mint addresses
- `filter_outcome_mints` - Filter token addresses by outcome mints
- `get_market_candlesticks` - Get market candlestick data
- `get_market_candlesticks_by_mint` - Get candlesticks by mint

### Orderbook API

- `get_orderbook` - Get orderbook data for a market by ticker
- `get_orderbook_by_mint` - Get orderbook data by mint address

### Trades API

- `get_trades` - Get paginated list of trades with filtering options
- `get_trades_by_mint` - Get trades for a market by mint address

### Live Data API

- `get_live_data` - Get live data for specified milestones
- `get_live_data_by_event` - Get live data by event ticker
- `get_live_data_by_mint` - Get live data by mint address

### Series API

- `get_series` - Get all series templates with filtering options
- `get_series_by_ticker` - Get a single series by ticker

### Tags API

- `get_tags_by_categories` - Get tags organized by series categories

### Sports API

- `get_filters_by_sports` - Get filtering options for each sport

### Search API

- `search_events` - Search for events by title or ticker

## Configuration

```rust
// Use default base URL
let client = DflowPredictionApiClient::with_default_url("api-key".to_string());

// Or use custom base URL
let client = DflowPredictionApiClient::new(
    "https://custom-api.example.com".to_string(),
    "api-key".to_string(),
);
```

## License

MIT
