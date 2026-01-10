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
