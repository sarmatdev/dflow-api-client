use serde::{Deserialize, Serialize};

// =============================================================================
// Common Types
// =============================================================================

/// Settlement source information for an event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementSource {
    pub name: String,
    pub url: String,
}

/// Market accounts containing Solana public keys
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketAccounts {
    #[serde(default)]
    pub market_ledger: Option<String>,
    #[serde(default)]
    pub yes_mint: Option<String>,
    #[serde(default)]
    pub no_mint: Option<String>,
    #[serde(default)]
    pub amm: Option<String>,
}

// =============================================================================
// Market Types
// =============================================================================

/// A prediction market
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    /// Market ticker ID
    pub ticker: String,
    /// Market title
    pub title: String,
    /// Market subtitle
    pub subtitle: String,
    /// Event ticker this market belongs to
    pub event_ticker: String,
    /// Market type (e.g., "binary")
    pub market_type: String,
    /// Market status (e.g., "active", "closed", "determined")
    pub status: String,
    /// Market result (e.g., "yes", "no", or empty if not determined)
    pub result: String,
    /// Whether the market can close early
    pub can_close_early: bool,
    /// Market open time (Unix timestamp in milliseconds)
    pub open_time: i64,
    /// Market close time (Unix timestamp in milliseconds)
    pub close_time: i64,
    /// Market expiration time (Unix timestamp in milliseconds)
    pub expiration_time: i64,
    /// Total trading volume
    pub volume: i64,
    /// Open interest
    pub open_interest: i64,
    /// Primary rules
    pub rules_primary: String,
    /// Yes outcome subtitle
    pub yes_sub_title: String,
    /// No outcome subtitle
    pub no_sub_title: String,
    /// Solana accounts related to this market
    pub accounts: MarketAccounts,
    /// Secondary rules (optional)
    #[serde(default)]
    pub rules_secondary: Option<String>,
    /// Early close condition description (optional)
    #[serde(default)]
    pub early_close_condition: Option<String>,
    /// Best yes ask price (optional)
    #[serde(default)]
    pub yes_ask: Option<String>,
    /// Best yes bid price (optional)
    #[serde(default)]
    pub yes_bid: Option<String>,
    /// Best no ask price (optional)
    #[serde(default)]
    pub no_ask: Option<String>,
    /// Best no bid price (optional)
    #[serde(default)]
    pub no_bid: Option<String>,
}

// =============================================================================
// Event Types
// =============================================================================

/// A prediction market event (can contain multiple markets)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Event ticker ID
    pub ticker: String,
    /// Event title
    pub title: String,
    /// Event subtitle
    pub subtitle: String,
    /// Series ticker this event belongs to
    pub series_ticker: String,
    /// Competition name (optional)
    #[serde(default)]
    pub competition: Option<String>,
    /// Competition scope (optional)
    #[serde(default)]
    pub competition_scope: Option<String>,
    /// Event image URL (optional)
    #[serde(default)]
    pub image_url: Option<String>,
    /// Total liquidity across all markets (optional)
    #[serde(default)]
    pub liquidity: Option<i64>,
    /// Total trading volume across all markets (optional)
    #[serde(default)]
    pub volume: Option<i64>,
    /// 24-hour trading volume (optional)
    #[serde(default)]
    pub volume24h: Option<i64>,
    /// Total open interest across all markets (optional)
    #[serde(default)]
    pub open_interest: Option<i64>,
    /// Strike date (Unix timestamp in milliseconds, optional)
    #[serde(default)]
    pub strike_date: Option<i64>,
    /// Strike period description (optional)
    #[serde(default)]
    pub strike_period: Option<String>,
    /// Settlement sources (optional)
    #[serde(default)]
    pub settlement_sources: Option<Vec<SettlementSource>>,
    /// Nested markets (optional, only included if requested)
    #[serde(default)]
    pub markets: Option<Vec<Market>>,
}

// =============================================================================
// Candlestick Types
// =============================================================================

/// OHLC candlestick data point
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    /// Candle start time (Unix timestamp in milliseconds)
    pub time: i64,
    /// Open price
    pub open: f64,
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
    /// Volume during this period
    #[serde(default)]
    pub volume: Option<i64>,
}

// =============================================================================
// Forecast Percentile Types
// =============================================================================

/// Forecast percentile history data point
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastPercentile {
    /// Timestamp (Unix timestamp in milliseconds)
    pub time: i64,
    /// Forecast percentile value (0-100)
    pub percentile: f64,
}

// =============================================================================
// Outcome Mint Types
// =============================================================================

// =============================================================================
// Response Types
// =============================================================================

/// Response for get_events endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsResponse {
    /// List of events
    pub events: Vec<Event>,
    /// Cursor for pagination (optional)
    #[serde(default)]
    pub cursor: Option<i32>,
}

/// Response for get_markets endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResponse {
    /// List of markets
    pub markets: Vec<Market>,
    /// Cursor for pagination (optional)
    #[serde(default)]
    pub cursor: Option<i32>,
}

/// Response for get_event_candlesticks endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlesticksResponse {
    /// List of candlesticks
    pub candlesticks: Vec<Candlestick>,
}

/// Response for forecast percentile history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastPercentileHistoryResponse {
    /// List of forecast percentile data points
    pub history: Vec<ForecastPercentile>,
}

/// Response for get_outcome_mints endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutcomeMintsResponse {
    /// List of outcome mint addresses
    pub mints: Vec<String>,
}

/// Response for filter_outcome_mints endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterOutcomeMintsResponse {
    /// List of addresses that are outcome mints
    pub outcome_mints: Vec<String>,
}

// =============================================================================
// Query Parameter Types
// =============================================================================

/// Sort options for events/markets queries
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortField {
    Volume,
    Volume24h,
    Liquidity,
    OpenInterest,
    StartDate,
}

impl SortField {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortField::Volume => "volume",
            SortField::Volume24h => "volume24h",
            SortField::Liquidity => "liquidity",
            SortField::OpenInterest => "openInterest",
            SortField::StartDate => "startDate",
        }
    }
}

/// Market status filter options
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    Initialized,
    Active,
    Inactive,
    Closed,
    Determined,
}

impl MarketStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MarketStatus::Initialized => "initialized",
            MarketStatus::Active => "active",
            MarketStatus::Inactive => "inactive",
            MarketStatus::Closed => "closed",
            MarketStatus::Determined => "determined",
        }
    }
}

/// Period interval options for candlesticks (in minutes)
#[derive(Debug, Clone, Copy)]
pub enum PeriodInterval {
    /// 1 minute
    OneMinute = 1,
    /// 1 hour (60 minutes)
    OneHour = 60,
    /// 1 day (1440 minutes)
    OneDay = 1440,
}

impl PeriodInterval {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

/// Query parameters for get_events endpoint
#[derive(Debug, Clone, Default)]
pub struct GetEventsParams {
    /// Maximum number of events to return
    pub limit: Option<i32>,
    /// Include nested markets in response
    pub with_nested_markets: Option<bool>,
    /// Pagination cursor (number of events to skip)
    pub cursor: Option<i32>,
    /// Filter by series tickers (comma-separated, max 25)
    pub series_tickers: Option<String>,
    /// Filter events that are initialized
    pub is_initialized: Option<bool>,
    /// Filter by market status
    pub status: Option<MarketStatus>,
    /// Sort field
    pub sort: Option<SortField>,
}

/// Query parameters for get_markets endpoint
#[derive(Debug, Clone, Default)]
pub struct GetMarketsParams {
    /// Maximum number of markets to return
    pub limit: Option<i32>,
    /// Pagination cursor (number of markets to skip)
    pub cursor: Option<i32>,
    /// Filter markets that are initialized (have a corresponding market ledger)
    pub is_initialized: Option<bool>,
    /// Filter by market status
    pub status: Option<MarketStatus>,
    /// Sort field
    pub sort: Option<SortField>,
}

/// Query parameters for get_outcome_mints endpoint
#[derive(Debug, Clone, Default)]
pub struct GetOutcomeMintsParams {
    /// Minimum close timestamp (Unix timestamp in seconds).
    /// Only markets with close_time >= min_close_ts will be included.
    pub min_close_ts: Option<i64>,
}

/// Query parameters for candlestick endpoints
#[derive(Debug, Clone, Default)]
pub struct GetCandlesticksParams {
    /// Start timestamp (Unix timestamp in seconds)
    pub start_ts: Option<i64>,
    /// End timestamp (Unix timestamp in seconds)
    pub end_ts: Option<i64>,
    /// Time period length of each candlestick in minutes (1, 60, or 1440)
    pub period_interval: Option<i32>,
}

/// Query parameters for forecast percentile history endpoint
#[derive(Debug, Clone, Default)]
pub struct GetForecastPercentileHistoryParams {
    /// Comma-separated list of percentile values (0-10000, max 10 values)
    pub percentiles: Option<String>,
    /// Start timestamp (Unix timestamp in seconds)
    pub start_ts: Option<i64>,
    /// End timestamp (Unix timestamp in seconds)
    pub end_ts: Option<i64>,
    /// Period interval in minutes (0, 1, 60, or 1440)
    pub period_interval: Option<i32>,
}

// =============================================================================
// Orderbook Types
// =============================================================================

/// Single level in an orderbook (bid or ask)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLevel {
    /// Price at this level
    pub price: f64,
    /// Quantity at this level
    pub quantity: i64,
}

/// Orderbook data for a market
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    /// Market ticker
    pub ticker: String,
    /// Yes outcome bids
    #[serde(default)]
    pub yes_bids: Vec<OrderLevel>,
    /// Yes outcome asks
    #[serde(default)]
    pub yes_asks: Vec<OrderLevel>,
    /// No outcome bids
    #[serde(default)]
    pub no_bids: Vec<OrderLevel>,
    /// No outcome asks
    #[serde(default)]
    pub no_asks: Vec<OrderLevel>,
}

// =============================================================================
// Trade Types
// =============================================================================

/// A single trade record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    /// Trade ID
    pub trade_id: String,
    /// Market ticker
    pub ticker: String,
    /// Trade count/quantity
    pub count: i64,
    /// Trade price (1-99)
    pub price: i64,
    /// Yes price (1-99)
    pub yes_price: i64,
    /// No price (1-99)
    pub no_price: i64,
    /// Yes price in dollars
    pub yes_price_dollars: String,
    /// No price in dollars
    pub no_price_dollars: String,
    /// Taker side ("yes" or "no")
    pub taker_side: String,
    /// Trade creation time (Unix timestamp in milliseconds)
    pub created_time: i64,
}

/// Response for get_trades endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradesResponse {
    /// List of trades
    pub trades: Vec<Trade>,
    /// Cursor for pagination
    #[serde(default)]
    pub cursor: Option<String>,
}

/// Query parameters for get_trades endpoint
#[derive(Debug, Clone, Default)]
pub struct GetTradesParams {
    /// Maximum number of trades to return (1-1000, default 100)
    pub limit: Option<i32>,
    /// Pagination cursor (trade ID) to start from
    pub cursor: Option<String>,
    /// Filter by market ticker
    pub ticker: Option<String>,
    /// Filter trades after this Unix timestamp
    pub min_ts: Option<i64>,
    /// Filter trades before this Unix timestamp
    pub max_ts: Option<i64>,
}

// =============================================================================
// Series Types
// =============================================================================

/// A series template for recurring events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    /// Series ticker ID
    pub ticker: String,
    /// Series title
    pub title: String,
    /// Series category (e.g., Politics, Economics, Entertainment)
    pub category: String,
    /// Contract URL
    #[serde(default)]
    pub contract_url: Option<String>,
    /// Contract terms URL
    #[serde(default)]
    pub contract_terms_url: Option<String>,
    /// Fee multiplier
    #[serde(default)]
    pub fee_multiplier: Option<i64>,
    /// Fee type
    #[serde(default)]
    pub fee_type: Option<String>,
    /// Frequency of events
    #[serde(default)]
    pub frequency: Option<String>,
    /// Product metadata (varies by series)
    #[serde(default)]
    pub product_metadata: Option<serde_json::Value>,
    /// Settlement sources
    #[serde(default)]
    pub settlement_sources: Option<Vec<SettlementSource>>,
    /// Tags associated with this series
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    /// Additional prohibitions
    #[serde(default)]
    pub additional_prohibitions: Option<Vec<String>>,
}

/// Response for get_series endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesResponse {
    /// List of series
    pub series: Vec<Series>,
}

/// Query parameters for get_series endpoint
#[derive(Debug, Clone, Default)]
pub struct GetSeriesParams {
    /// Filter series by category (e.g., Politics, Economics, Entertainment)
    pub category: Option<String>,
    /// Filter series by tags (comma-separated list)
    pub tags: Option<String>,
    /// Filter series that are initialized (have a corresponding market ledger)
    pub is_initialized: Option<bool>,
    /// Filter series by market status
    pub status: Option<MarketStatus>,
}

// =============================================================================
// Tags Types
// =============================================================================

/// Response for get_tags_by_categories endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagsByCategoriesResponse {
    /// Map of category to list of tags
    pub tags_by_categories: std::collections::HashMap<String, Vec<String>>,
}

// =============================================================================
// Sports Types
// =============================================================================

/// Response for get_filters_by_sports endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FiltersBySportsResponse {
    /// Filters organized by sport
    pub filters_by_sports: serde_json::Value,
    /// Ordered list of sports
    pub sport_ordering: Vec<String>,
}

// =============================================================================
// Search Types
// =============================================================================

/// Sort order for search results
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

/// Response for search endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    /// List of matching events with nested markets
    pub events: Vec<Event>,
    /// Cursor for pagination
    #[serde(default)]
    pub cursor: Option<i32>,
}

/// Query parameters for search endpoint
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    /// The query string to search for (required)
    pub q: String,
    /// Field to sort by
    pub sort: Option<SortField>,
    /// Sort order (asc or desc)
    pub order: Option<SortOrder>,
    /// Maximum number of results to return
    pub limit: Option<i32>,
    /// Cursor for pagination
    pub cursor: Option<i32>,
    /// Include nested markets in response
    pub with_nested_markets: Option<bool>,
    /// Include market account information
    pub with_market_accounts: Option<bool>,
}

// =============================================================================
// Live Data Types
// =============================================================================

/// Live data response (passthrough from Kalshi API)
/// The structure varies based on the milestone type, so we use a generic JSON value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDataResponse {
    /// Live data for the requested milestones
    #[serde(flatten)]
    pub data: serde_json::Value,
}
