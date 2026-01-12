use thiserror::Error;

/// Errors that can occur when interacting with the DFlow Swap API.
#[derive(Debug, Error)]
pub enum DflowSwapApiError {
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

    /// No route found for the swap
    #[error("No route found: {0}")]
    NoRouteFound(String),
}

/// API error response body structure
#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorResponse {
    pub error: Option<String>,
    pub message: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
}

impl DflowSwapApiError {
    /// Create an API error from status code and response body
    pub fn from_response(status_code: u16, body: &str) -> Self {
        match status_code {
            401 => DflowSwapApiError::Unauthorized,
            404 => DflowSwapApiError::NotFound(body.to_string()),
            429 => DflowSwapApiError::RateLimited,
            _ => {
                let message = serde_json::from_str::<ApiErrorResponse>(body)
                    .ok()
                    .and_then(|e| e.message.or(e.error))
                    .unwrap_or_else(|| body.to_string());

                DflowSwapApiError::ApiError {
                    status_code,
                    message,
                }
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, DflowSwapApiError>;
