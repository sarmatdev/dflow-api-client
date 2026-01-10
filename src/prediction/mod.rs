pub mod error;
pub mod types;

pub use error::{DflowPredictionApiError, Result};
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
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

impl DflowPredictionApiClient {
    /// Create a new DFlow Prediction API client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL for the API (e.g., "https://prediction-markets-api.dflow.net")
    /// * `api_key` - API key for authentication
    pub fn new(base_url: String, api_key: String) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "x-api-key",
            HeaderValue::from_str(&api_key).expect("Invalid API key"),
        );

        let http_client = Client::builder()
            .default_headers(default_headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http_client,
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

    /// Build query string from optional parameters
    fn build_query_string(&self, params: &[(&str, Option<String>)]) -> String {
        let query_parts: Vec<String> = params
            .iter()
            .filter_map(|(key, value)| {
                value.as_ref().map(|v| format!("{}={}", key, v))
            })
            .collect();

        if query_parts.is_empty() {
            String::new()
        } else {
            format!("?{}", query_parts.join("&"))
        }
    }

    /// Make a GET request to the API
    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self.http_client.get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(DflowPredictionApiError::from_response(
                status.as_u16(),
                &body,
            ));
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| {
            DflowPredictionApiError::ParseError(format!("{}: {}", e, body))
        })
    }

    /// Make a POST request to the API
    async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self.http_client.post(&url).json(body).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(DflowPredictionApiError::from_response(
                status.as_u16(),
                &body,
            ));
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| {
            DflowPredictionApiError::ParseError(format!("{}: {}", e, body))
        })
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
        let query = self.build_query_string(&[(
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

        let query = self.build_query_string(&[
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

        let query = self.build_query_string(&[
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

        let query = self.build_query_string(&[
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

        let query = self.build_query_string(&[
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

        let query = self.build_query_string(&[
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

        let query = self.build_query_string(&[(
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

        let query = self.build_query_string(&[
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

        let query = self.build_query_string(&[
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
}
