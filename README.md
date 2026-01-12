# DFlow API Client

A Rust client library for interacting with the [DFlow API](https://pond.dflow.net/introduction).

## Features

- **REST API** - Full coverage of the DFlow Prediction Market Metadata API
- **WebSocket** - Real-time streaming of prices, trades, and orderbook updates (optional feature)
- **Swap API** - Token swap functionality via imperative or declarative flows

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dflow-api-client = { git = "https://github.com/sarmatdev/dflow-api-client" }
tokio = { version = "1", features = ["full"] }
```

### With WebSocket Support

```toml
[dependencies]
dflow-api-client = { git = "https://github.com/sarmatdev/dflow-api-client", features = ["websocket"] }
tokio = { version = "1", features = ["full"] }
futures-util = "0.3"
```

## Usage

### REST API

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

### WebSocket API

Stream real-time market data using the WebSocket client:

```rust
use dflow_api_client::prediction::websocket::DflowPredictionWsClient;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to WebSocket
    let client = DflowPredictionWsClient::connect().await?;

    // Subscribe to all price updates
    let (mut stream, unsubscribe) = client.prices_subscribe_all().await?;

    // Process incoming updates
    while let Some(update) = stream.next().await {
        println!(
            "Market: {} | YES bid: {:?} ask: {:?}",
            update.market_ticker, update.yes_bid, update.yes_ask
        );
    }

    // Cleanup
    unsubscribe().await;
    client.shutdown().await?;

    Ok(())
}
```

#### Subscribe to Specific Tickers

```rust
// Subscribe to specific market tickers
let (mut stream, unsubscribe) = client
    .prices_subscribe_tickers(vec![
        "BTCD-25DEC0313-T92749.99".to_string(),
        "SPX-25DEC0313-T5000".to_string(),
    ])
    .await?;
```

#### Multiple Channel Subscriptions

```rust
// Subscribe to trades
let (mut trades, _) = client.trades_subscribe_all().await?;

// Subscribe to orderbook updates
let (mut orderbook, _) = client.orderbook_subscribe_all().await?;

// Or subscribe to specific tickers
let (mut trades, _) = client.trades_subscribe_tickers(vec!["TICKER".to_string()]).await?;
let (mut orderbook, _) = client.orderbook_subscribe_tickers(vec!["TICKER".to_string()]).await?;
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

### WebSocket API (requires `websocket` feature)

**Prices Channel**
- `prices_subscribe_all` - Subscribe to price updates for all markets
- `prices_subscribe_tickers` - Subscribe to price updates for specific tickers

**Trades Channel**
- `trades_subscribe_all` - Subscribe to trade updates for all markets
- `trades_subscribe_tickers` - Subscribe to trade updates for specific tickers

**Orderbook Channel**
- `orderbook_subscribe_all` - Subscribe to orderbook updates for all markets
- `orderbook_subscribe_tickers` - Subscribe to orderbook updates for specific tickers

## Configuration

### REST API

```rust
// Use default base URL
let client = DflowPredictionApiClient::with_default_url("api-key".to_string());

// Or use custom base URL
let client = DflowPredictionApiClient::new(
    "https://custom-api.example.com".to_string(),
    "api-key".to_string(),
);
```

### WebSocket API

```rust
use dflow_api_client::prediction::websocket::DflowPredictionWsClient;

// Use default WebSocket URL
let client = DflowPredictionWsClient::connect().await?;

// Or use custom WebSocket URL
let client = DflowPredictionWsClient::connect_with_url(
    "wss://custom-ws.example.com/api/v1/ws"
).await?;
```

## License

MIT
