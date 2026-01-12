pub mod error;
pub mod types;

pub use error::{DflowSwapApiError, Result};
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
pub use types::*;

/// Default base URL for the DFlow Swap API
pub const DEFAULT_BASE_URL: &str = "https://swap-api.dflow.net";

/// Client for interacting with the DFlow Swap API.
///
/// Supports both imperative (quote + swap) and declarative (intent-based) swap flows.
///
/// # Example
///
/// ```no_run
/// use dflow_api_client::swap::{DflowSwapApiClient, GetQuoteParams};
///
/// #[tokio::main]
/// async fn main() {
///     let client = DflowSwapApiClient::new(
///         "https://swap-api.dflow.net".to_string(),
///         "your-api-key".to_string(),
///     );
///
///     // Get a quote for swapping SOL to USDC
///     let params = GetQuoteParams {
///         input_mint: "So11111111111111111111111111111111111111112".to_string(),
///         output_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
///         amount: "1000000000".to_string(), // 1 SOL in lamports
///         slippage_bps: Some(50), // 0.5% slippage
///         ..Default::default()
///     };
///
///     let quote = client.get_quote(params).await.unwrap();
///     println!("Quote: {} -> {}", quote.in_amount, quote.out_amount);
/// }
/// ```
#[derive(Clone)]
pub struct DflowSwapApiClient {
    http_client: Client,
    base_url: String,
}

impl DflowSwapApiClient {
    /// Create a new DFlow Swap API client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL for the API (e.g., "https://swap-api.dflow.net")
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
            return Err(DflowSwapApiError::from_response(
                status.as_u16(),
                &body,
            ));
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| {
            DflowSwapApiError::ParseError(format!("{}: {}", e, body))
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
            return Err(DflowSwapApiError::from_response(
                status.as_u16(),
                &body,
            ));
        }

        let body = response.text().await?;
        serde_json::from_str(&body).map_err(|e| {
            DflowSwapApiError::ParseError(format!("{}: {}", e, body))
        })
    }

    // =========================================================================
    // Imperative Swap API Endpoints
    // =========================================================================

    /// Get a quote for a token swap.
    ///
    /// This endpoint returns pricing information for swapping between two tokens,
    /// including the expected output amount, price impact, and route plan.
    ///
    /// # Arguments
    ///
    /// * `params` - Quote parameters including input/output mints and amount
    ///
    /// # Returns
    ///
    /// Quote response with pricing and route information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dflow_api_client::swap::{DflowSwapApiClient, GetQuoteParams};
    ///
    /// # async fn example() {
    /// let client = DflowSwapApiClient::with_default_url("api-key".to_string());
    ///
    /// let params = GetQuoteParams {
    ///     input_mint: "So11111111111111111111111111111111111111112".to_string(),
    ///     output_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
    ///     amount: "1000000000".to_string(),
    ///     slippage_bps: Some(50),
    ///     ..Default::default()
    /// };
    ///
    /// let quote = client.get_quote(params).await.unwrap();
    /// # }
    /// ```
    pub async fn get_quote(
        &self,
        params: GetQuoteParams,
    ) -> Result<QuoteResponse> {
        let query = self.build_query_string(&[
            ("inputMint", Some(params.input_mint)),
            ("outputMint", Some(params.output_mint)),
            ("amount", Some(params.amount)),
            ("slippageBps", params.slippage_bps.map(|v| v.to_string())),
            ("exactIn", params.exact_in.map(|v| v.to_string())),
            ("userPublicKey", params.user_public_key),
        ]);

        self.get(&format!("/quote{}", query)).await
    }

    /// Create a swap transaction from a quote.
    ///
    /// This endpoint takes a quote response and returns a serialized transaction
    /// that the user can sign and submit to the Solana network.
    ///
    /// # Arguments
    ///
    /// * `request` - Swap request containing the quote and user's public key
    ///
    /// # Returns
    ///
    /// Swap response with the serialized transaction.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dflow_api_client::swap::{DflowSwapApiClient, GetQuoteParams, SwapRequest};
    ///
    /// # async fn example() {
    /// let client = DflowSwapApiClient::with_default_url("api-key".to_string());
    ///
    /// // First get a quote
    /// let quote_params = GetQuoteParams {
    ///     input_mint: "So11111111111111111111111111111111111111112".to_string(),
    ///     output_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
    ///     amount: "1000000000".to_string(),
    ///     slippage_bps: Some(50),
    ///     ..Default::default()
    /// };
    /// let quote = client.get_quote(quote_params).await.unwrap();
    ///
    /// // Then create the swap transaction
    /// let swap_request = SwapRequest {
    ///     quote_response: quote,
    ///     user_public_key: "YourWalletPublicKey".to_string(),
    ///     wrap_and_unwrap_sol: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let swap = client.create_swap(swap_request).await.unwrap();
    /// println!("Transaction: {}", swap.swap_transaction);
    /// # }
    /// ```
    pub async fn create_swap(
        &self,
        request: SwapRequest,
    ) -> Result<SwapResponse> {
        self.post("/swap", &request).await
    }

    // =========================================================================
    // Declarative Swap API Endpoints
    // =========================================================================

    /// Get an intent quote for a declarative swap.
    ///
    /// Intent-based swaps allow for more flexible execution, where the actual
    /// swap is performed by solvers who compete to provide the best execution.
    ///
    /// # Arguments
    ///
    /// * `params` - Intent parameters including input/output mints and amount
    ///
    /// # Returns
    ///
    /// Intent response with quote and intent ID for submission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dflow_api_client::swap::{DflowSwapApiClient, GetIntentParams};
    ///
    /// # async fn example() {
    /// let client = DflowSwapApiClient::with_default_url("api-key".to_string());
    ///
    /// let params = GetIntentParams {
    ///     input_mint: "So11111111111111111111111111111111111111112".to_string(),
    ///     output_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
    ///     amount: "1000000000".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let intent = client.get_intent(params).await.unwrap();
    /// println!("Intent ID: {}", intent.intent_id);
    /// # }
    /// ```
    pub async fn get_intent(
        &self,
        params: GetIntentParams,
    ) -> Result<IntentResponse> {
        let query = self.build_query_string(&[
            ("inputMint", Some(params.input_mint)),
            ("outputMint", Some(params.output_mint)),
            ("amount", Some(params.amount)),
            ("exactIn", params.exact_in.map(|v| v.to_string())),
            ("userPublicKey", params.user_public_key),
        ]);

        self.get(&format!("/intent{}", query)).await
    }

    /// Submit an intent for execution.
    ///
    /// After getting an intent quote, submit it with a signature to have
    /// solvers execute the swap on your behalf.
    ///
    /// # Arguments
    ///
    /// * `request` - Submit intent request with intent ID and signature
    ///
    /// # Returns
    ///
    /// Submit intent response with execution status.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dflow_api_client::swap::{DflowSwapApiClient, GetIntentParams, SubmitIntentRequest};
    ///
    /// # async fn example() {
    /// let client = DflowSwapApiClient::with_default_url("api-key".to_string());
    ///
    /// // First get an intent
    /// let intent_params = GetIntentParams {
    ///     input_mint: "So11111111111111111111111111111111111111112".to_string(),
    ///     output_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
    ///     amount: "1000000000".to_string(),
    ///     ..Default::default()
    /// };
    /// let intent = client.get_intent(intent_params).await.unwrap();
    ///
    /// // Then submit the intent with a signature
    /// let submit_request = SubmitIntentRequest {
    ///     intent_id: intent.intent_id,
    ///     user_public_key: "YourWalletPublicKey".to_string(),
    ///     signature: "YourSignature".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let result = client.submit_intent(submit_request).await.unwrap();
    /// println!("Status: {}", result.status);
    /// # }
    /// ```
    pub async fn submit_intent(
        &self,
        request: SubmitIntentRequest,
    ) -> Result<SubmitIntentResponse> {
        self.post("/submit-intent", &request).await
    }
}
