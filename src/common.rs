//! Common utilities and types shared across DFlow API clients.

use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use thiserror::Error;

// =========================================================================
// Error Types
// =========================================================================

/// API error response body structure
#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorResponse {
    pub error: Option<String>,
    pub message: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
}

/// Errors that can occur when interacting with the DFlow APIs.
#[derive(Debug, Error)]
pub enum DflowApiError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error (status {status_code}): {message}")]
    ApiError { status_code: u16, message: String },

    /// Failed to parse response body
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// Invalid parameter provided
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Authentication failed
    #[error("Authentication failed: invalid or missing API key")]
    Unauthorized,

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimited,

    /// No route found for the swap (Swap API specific)
    #[error("No route found: {0}")]
    NoRouteFound(String),
}

impl DflowApiError {
    /// Create an API error from status code and response body
    pub fn from_response(status_code: u16, body: &str) -> Self {
        match status_code {
            401 => DflowApiError::Unauthorized,
            404 => DflowApiError::NotFound(body.to_string()),
            429 => DflowApiError::RateLimited,
            _ => {
                let message = serde_json::from_str::<ApiErrorResponse>(body)
                    .ok()
                    .and_then(|e| e.message.or(e.error))
                    .unwrap_or_else(|| body.to_string());

                DflowApiError::ApiError {
                    status_code,
                    message,
                }
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, DflowApiError>;

// =========================================================================
// HTTP Utilities
// =========================================================================

/// Build query string from optional parameters.
///
/// # Arguments
///
/// * `params` - Slice of key-value pairs where values are optional
///
/// # Returns
///
/// A query string starting with `?` if there are any parameters, or empty string.
pub fn build_query_string(params: &[(&str, Option<String>)]) -> String {
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

/// Create an HTTP client with the given API key in the default headers.
///
/// # Arguments
///
/// * `api_key` - API key for authentication
///
/// # Returns
///
/// A configured `reqwest::Client` with the API key header set.
pub fn create_http_client(api_key: &str) -> Client {
    let mut default_headers = HeaderMap::new();
    default_headers.insert(
        "x-api-key",
        HeaderValue::from_str(api_key).expect("Invalid API key"),
    );

    Client::builder()
        .default_headers(default_headers)
        .build()
        .expect("Failed to build HTTP client")
}

/// Trait for common DFlow API client functionality.
///
/// This trait provides the core HTTP methods (`get` and `post`) that are
/// shared across different DFlow API clients.
#[allow(async_fn_in_trait)]
pub trait DflowHttpClient {
    /// Get the HTTP client
    fn http_client(&self) -> &Client;

    /// Get the base URL
    fn base_url(&self) -> &str;

    /// Make a GET request to the API
    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url(), endpoint);

        let response = self.http_client().get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(DflowApiError::from_response(status.as_u16(), &body));
        }

        let body = response.text().await?;
        serde_json::from_str(&body)
            .map_err(|e| DflowApiError::ParseError(format!("{}: {}", e, body)))
    }

    /// Make a POST request to the API
    async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url(), endpoint);

        let response = self.http_client().post(&url).json(body).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(DflowApiError::from_response(status.as_u16(), &body));
        }

        let body = response.text().await?;
        serde_json::from_str(&body)
            .map_err(|e| DflowApiError::ParseError(format!("{}: {}", e, body)))
    }
}
