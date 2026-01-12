pub mod types;

#[cfg(feature = "websocket")]
pub mod websocket;

use crate::common::{DflowHttpClient, build_query_string, create_http_client};

/// Error type for the DFlow Prediction Market API.
pub type DflowPredictionApiError = crate::common::DflowApiError;
/// Result type for the DFlow Prediction Market API.
pub type Result<T> = crate::common::Result<T>;
use reqwest::Client;
pub use types::*;

/// Default base URL for the DFlow Prediction Market API
pub const DEFAULT_BASE_URL: &str = "https://prediction-markets-api.dflow.net";

/// Client for interacting with the DFlow Prediction Market Metadata API.
///
/// # Example
///
/// ```no_run
/// use dflow_api_client::prediction::DflowPredictionApiClient;
///
/// #[tokio::main]
/// async fn main() {
///     let client = DflowPredictionApiClient::new(
///         "https://prediction-markets-api.dflow.net".to_string(),
///         "your-api-key".to_string(),
///     );
///
///     let event = client.get_event("EVENT_TICKER", None).await.unwrap();
///     println!("Event: {:?}", event);
/// }
/// ```
#[derive(Clone)]
pub struct DflowPredictionApiClient {
    http_client: Client,
    base_url: String,
}

impl DflowHttpClient for DflowPredictionApiClient {
    fn http_client(&self) -> &Client {
        &self.http_client
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl DflowPredictionApiClient {
    /// Create a new DFlow Prediction API client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL for the API (e.g., "https://prediction-markets-api.dflow.net")
    /// * `api_key` - API key for authentication
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            http_client: create_http_client(&api_key),
            base_url,
        }
    }

    /// Create a new client with the default base URL.
    ///
    /// # Arguments
    ///
    /// * `api_key` - API key for authentication
    pub fn with_default_url(api_key: String) -> Self {
        Self::new(DEFAULT_BASE_URL.to_string(), api_key)
    }

    // =========================================================================
    // Events API Endpoints
    // =========================================================================

    /// Get a single event by its ticker ID.
    ///
    /// # Arguments
    ///
    /// * `event_id` - Event ticker ID
    /// * `with_nested_markets` - Include nested markets in response
    ///
    /// # Returns
    ///
    /// The event with the given ticker ID.
    pub async fn get_event(
        &self,
        event_id: &str,
        with_nested_markets: Option<bool>,
    ) -> Result<Event> {
        let query = build_query_string(&[(
            "withNestedMarkets",
            with_nested_markets.map(|v| v.to_string()),
        )]);

        self.get(&format!("/api/v1/event/{}{}", event_id, query))
            .await
    }

    /// Get a paginated list of events.
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// A paginated list of events.
    pub async fn get_events(
        &self,
        params: Option<GetEventsParams>,
    ) -> Result<EventsResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("limit", params.limit.map(|v| v.to_string())),
            (
                "withNestedMarkets",
                params.with_nested_markets.map(|v| v.to_string()),
            ),
            ("cursor", params.cursor.map(|v| v.to_string())),
            ("seriesTickers", params.series_tickers),
            (
                "isInitialized",
                params.is_initialized.map(|v| v.to_string()),
            ),
            ("status", params.status.map(|v| v.as_str().to_string())),
            ("sort", params.sort.map(|v| v.as_str().to_string())),
        ]);

        self.get(&format!("/api/v1/events{}", query)).await
    }

    /// Get forecast percentile history for an event.
    ///
    /// # Arguments
    ///
    /// * `series_ticker` - Series ticker
    /// * `event_id` - Event ticker ID
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    ///
    /// Forecast percentile history for the event.
    pub async fn get_event_forecast_percentile_history(
        &self,
        series_ticker: &str,
        event_id: &str,
        params: Option<GetForecastPercentileHistoryParams>,
    ) -> Result<ForecastPercentileHistoryResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("percentiles", params.percentiles),
            ("startTs", params.start_ts.map(|v| v.to_string())),
            ("endTs", params.end_ts.map(|v| v.to_string())),
            (
                "periodInterval",
                params.period_interval.map(|v| v.to_string()),
            ),
        ]);

        self.get(&format!(
            "/api/v1/event/{series_ticker}/{event_id}/forecast_percentile_history{}",
            query
        ))
        .await
    }

    /// Get forecast percentile history by mint address.
    ///
    /// # Arguments
    ///
    /// * `mint` - Mint address
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    ///
    /// Forecast percentile history for the mint.
    pub async fn get_event_forecast_percentile_history_by_mint(
        &self,
        mint: &str,
        params: Option<GetForecastPercentileHistoryParams>,
    ) -> Result<ForecastPercentileHistoryResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("percentiles", params.percentiles),
            ("startTs", params.start_ts.map(|v| v.to_string())),
            ("endTs", params.end_ts.map(|v| v.to_string())),
            (
                "periodInterval",
                params.period_interval.map(|v| v.to_string()),
            ),
        ]);

        self.get(&format!(
            "/api/v1/event/by-mint/{mint}/forecast_percentile_history{}",
            query
        ))
        .await
    }

    /// Get candlestick data for an event.
    ///
    /// # Arguments
    ///
    /// * `ticker` - Event ticker
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    ///
    /// Candlestick data for the event.
    pub async fn get_event_candlesticks(
        &self,
        ticker: &str,
        params: Option<GetCandlesticksParams>,
    ) -> Result<CandlesticksResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("startTs", params.start_ts.map(|v| v.to_string())),
            ("endTs", params.end_ts.map(|v| v.to_string())),
            (
                "periodInterval",
                params.period_interval.map(|v| v.to_string()),
            ),
        ]);

        self.get(&format!("/api/v1/event/{ticker}/candlesticks{}", query))
            .await
    }

    // =========================================================================
    // Markets API Endpoints
    // =========================================================================

    /// Get a single market by its ticker ID.
    ///
    /// # Arguments
    ///
    /// * `market_id` - Market ticker ID
    ///
    /// # Returns
    ///
    /// The market with the given ticker ID.
    pub async fn get_market(&self, market_id: &str) -> Result<Market> {
        self.get(&format!("/api/v1/market/{}", market_id)).await
    }

    /// Get a market by its mint address.
    ///
    /// # Arguments
    ///
    /// * `mint` - Mint address (yes or no outcome mint)
    ///
    /// # Returns
    ///
    /// The market associated with the mint.
    pub async fn get_market_by_mint(&self, mint: &str) -> Result<Market> {
        self.get(&format!("/api/v1/market/by-mint/{}", mint)).await
    }

    /// Get a paginated list of markets.
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// A paginated list of markets.
    pub async fn get_markets(
        &self,
        params: Option<GetMarketsParams>,
    ) -> Result<MarketsResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("limit", params.limit.map(|v| v.to_string())),
            ("cursor", params.cursor.map(|v| v.to_string())),
            (
                "isInitialized",
                params.is_initialized.map(|v| v.to_string()),
            ),
            ("status", params.status.map(|v| v.as_str().to_string())),
            ("sort", params.sort.map(|v| v.as_str().to_string())),
        ]);

        self.get(&format!("/api/v1/markets{}", query)).await
    }

    /// Get multiple markets by their ticker IDs in a single request.
    ///
    /// # Arguments
    ///
    /// * `tickers` - List of market ticker IDs
    ///
    /// # Returns
    ///
    /// List of markets matching the given tickers.
    pub async fn get_markets_batch(
        &self,
        tickers: &[String],
    ) -> Result<MarketsResponse> {
        #[derive(serde::Serialize)]
        struct BatchRequest {
            tickers: Vec<String>,
        }

        self.post(
            "/api/v1/markets/batch",
            &BatchRequest {
                tickers: tickers.to_vec(),
            },
        )
        .await
    }

    /// Get all outcome mints from supported markets.
    ///
    /// Returns a flat list of all yes_mint and no_mint pubkeys from all supported markets.
    /// Optionally filters by minimum close timestamp.
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    ///
    /// List of all outcome mint addresses.
    pub async fn get_outcome_mints(
        &self,
        params: Option<GetOutcomeMintsParams>,
    ) -> Result<OutcomeMintsResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[(
            "minCloseTs",
            params.min_close_ts.map(|v| v.to_string()),
        )]);

        self.get(&format!("/api/v1/outcome_mints{}", query)).await
    }

    /// Filter and validate a list of token addresses.
    ///
    /// Accepts a list of token addresses (max 200) and returns only those
    /// that are outcome mints (yes_mint or no_mint) from supported markets.
    ///
    /// # Arguments
    ///
    /// * `addresses` - List of token addresses to filter (max 200)
    ///
    /// # Returns
    ///
    /// Filtered list of addresses that are outcome mints.
    pub async fn filter_outcome_mints(
        &self,
        addresses: &[String],
    ) -> Result<FilterOutcomeMintsResponse> {
        #[derive(serde::Serialize)]
        struct FilterRequest {
            addresses: Vec<String>,
        }

        self.post(
            "/api/v1/filter_outcome_mints",
            &FilterRequest {
                addresses: addresses.to_vec(),
            },
        )
        .await
    }

    /// Get candlestick data for a market.
    ///
    /// # Arguments
    ///
    /// * `ticker` - Market ticker
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    ///
    /// Candlestick data for the market.
    pub async fn get_market_candlesticks(
        &self,
        ticker: &str,
        params: Option<GetCandlesticksParams>,
    ) -> Result<CandlesticksResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("startTs", params.start_ts.map(|v| v.to_string())),
            ("endTs", params.end_ts.map(|v| v.to_string())),
            (
                "periodInterval",
                params.period_interval.map(|v| v.to_string()),
            ),
        ]);

        self.get(&format!("/api/v1/market/{ticker}/candlesticks{}", query))
            .await
    }

    /// Get candlestick data for a market by mint address.
    ///
    /// # Arguments
    ///
    /// * `mint` - Mint address
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    ///
    /// Candlestick data for the market associated with the mint.
    pub async fn get_market_candlesticks_by_mint(
        &self,
        mint: &str,
        params: Option<GetCandlesticksParams>,
    ) -> Result<CandlesticksResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("startTs", params.start_ts.map(|v| v.to_string())),
            ("endTs", params.end_ts.map(|v| v.to_string())),
            (
                "periodInterval",
                params.period_interval.map(|v| v.to_string()),
            ),
        ]);

        self.get(&format!(
            "/api/v1/market/by-mint/{mint}/candlesticks{}",
            query
        ))
        .await
    }

    // =========================================================================
    // Orderbook API Endpoints
    // =========================================================================

    /// Get orderbook data for a market by its ticker.
    ///
    /// # Arguments
    ///
    /// * `market_ticker` - Market ticker ID
    ///
    /// # Returns
    ///
    /// Orderbook data for the market.
    pub async fn get_orderbook(
        &self,
        market_ticker: &str,
    ) -> Result<Orderbook> {
        self.get(&format!("/api/v1/orderbook/{}", market_ticker))
            .await
    }

    /// Get orderbook data for a market by mint address.
    ///
    /// # Arguments
    ///
    /// * `mint` - Mint address (yes or no outcome mint)
    ///
    /// # Returns
    ///
    /// Orderbook data for the market associated with the mint.
    pub async fn get_orderbook_by_mint(&self, mint: &str) -> Result<Orderbook> {
        self.get(&format!("/api/v1/orderbook/by-mint/{}", mint))
            .await
    }

    // =========================================================================
    // Trades API Endpoints
    // =========================================================================

    /// Get a paginated list of trades.
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// A paginated list of trades.
    pub async fn get_trades(
        &self,
        params: Option<GetTradesParams>,
    ) -> Result<TradesResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("limit", params.limit.map(|v| v.to_string())),
            ("cursor", params.cursor),
            ("ticker", params.ticker),
            ("minTs", params.min_ts.map(|v| v.to_string())),
            ("maxTs", params.max_ts.map(|v| v.to_string())),
        ]);

        self.get(&format!("/api/v1/trades{}", query)).await
    }

    /// Get trades for a market by mint address.
    ///
    /// # Arguments
    ///
    /// * `mint` - Mint address (yes or no outcome mint)
    /// * `params` - Query parameters for filtering and pagination
    ///
    /// # Returns
    ///
    /// A list of trades for the market associated with the mint.
    pub async fn get_trades_by_mint(
        &self,
        mint: &str,
        params: Option<GetTradesParams>,
    ) -> Result<TradesResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("limit", params.limit.map(|v| v.to_string())),
            ("cursor", params.cursor),
            ("minTs", params.min_ts.map(|v| v.to_string())),
            ("maxTs", params.max_ts.map(|v| v.to_string())),
        ]);

        self.get(&format!("/api/v1/trades/by-mint/{}{}", mint, query))
            .await
    }

    // =========================================================================
    // Live Data API Endpoints
    // =========================================================================

    /// Get live data for specified milestones.
    ///
    /// Relays live data from the Kalshi API for one or more milestones.
    ///
    /// # Arguments
    ///
    /// * `milestone_ids` - Array of milestone IDs (max 100)
    ///
    /// # Returns
    ///
    /// Live data for the requested milestones.
    pub async fn get_live_data(
        &self,
        milestone_ids: &[String],
    ) -> Result<LiveDataResponse> {
        let ids_param = milestone_ids.join(",");
        let query = build_query_string(&[("milestoneIds", Some(ids_param))]);

        self.get(&format!("/api/v1/live_data{}", query)).await
    }

    /// Get live data for an event by its ticker.
    ///
    /// # Arguments
    ///
    /// * `event_ticker` - Event ticker ID
    ///
    /// # Returns
    ///
    /// Live data for the event.
    pub async fn get_live_data_by_event(
        &self,
        event_ticker: &str,
    ) -> Result<LiveDataResponse> {
        self.get(&format!("/api/v1/live_data/by-event/{}", event_ticker))
            .await
    }

    /// Get live data for a market by mint address.
    ///
    /// # Arguments
    ///
    /// * `mint` - Mint address (yes or no outcome mint)
    ///
    /// # Returns
    ///
    /// Live data for the market associated with the mint.
    pub async fn get_live_data_by_mint(
        &self,
        mint: &str,
    ) -> Result<LiveDataResponse> {
        self.get(&format!("/api/v1/live_data/by-mint/{}", mint))
            .await
    }

    // =========================================================================
    // Series API Endpoints
    // =========================================================================

    /// Get all series templates.
    ///
    /// Returns all series templates available. A series represents a template for recurring events.
    ///
    /// # Arguments
    ///
    /// * `params` - Query parameters for filtering
    ///
    /// # Returns
    ///
    /// List of series.
    pub async fn get_series(
        &self,
        params: Option<GetSeriesParams>,
    ) -> Result<SeriesResponse> {
        let params = params.unwrap_or_default();

        let query = build_query_string(&[
            ("category", params.category),
            ("tags", params.tags),
            (
                "isInitialized",
                params.is_initialized.map(|v| v.to_string()),
            ),
            ("status", params.status.map(|v| v.as_str().to_string())),
        ]);

        self.get(&format!("/api/v1/series{}", query)).await
    }

    /// Get a single series by its ticker.
    ///
    /// # Arguments
    ///
    /// * `series_ticker` - Series ticker ID
    ///
    /// # Returns
    ///
    /// The series with the given ticker ID.
    pub async fn get_series_by_ticker(
        &self,
        series_ticker: &str,
    ) -> Result<Series> {
        self.get(&format!("/api/v1/series/{}", series_ticker)).await
    }

    // =========================================================================
    // Tags API Endpoints
    // =========================================================================

    /// Get tags organized by series categories.
    ///
    /// Returns a mapping of series categories to their associated tags.
    ///
    /// # Returns
    ///
    /// Tags grouped by categories.
    pub async fn get_tags_by_categories(
        &self,
    ) -> Result<TagsByCategoriesResponse> {
        self.get("/api/v1/tags_by_categories").await
    }

    // =========================================================================
    // Sports API Endpoints
    // =========================================================================

    /// Get filtering options available for each sport.
    ///
    /// Returns filtering options including scopes and competitions for each sport.
    ///
    /// # Returns
    ///
    /// Filters organized by sport.
    pub async fn get_filters_by_sports(
        &self,
    ) -> Result<FiltersBySportsResponse> {
        self.get("/api/v1/filters_by_sports").await
    }

    // =========================================================================
    // Search API Endpoints
    // =========================================================================

    /// Search for events by title or ticker.
    ///
    /// Returns events with nested markets which match the search query.
    ///
    /// # Arguments
    ///
    /// * `params` - Search parameters including query string
    ///
    /// # Returns
    ///
    /// Matching events with optional nested markets.
    pub async fn search_events(
        &self,
        params: SearchParams,
    ) -> Result<SearchResponse> {
        let query = build_query_string(&[
            ("q", Some(params.q)),
            ("sort", params.sort.map(|v| v.as_str().to_string())),
            ("order", params.order.map(|v| v.as_str().to_string())),
            ("limit", params.limit.map(|v| v.to_string())),
            ("cursor", params.cursor.map(|v| v.to_string())),
            (
                "withNestedMarkets",
                params.with_nested_markets.map(|v| v.to_string()),
            ),
            (
                "withMarketAccounts",
                params.with_market_accounts.map(|v| v.to_string()),
            ),
        ]);

        self.get(&format!("/api/v1/search{}", query)).await
    }
}
