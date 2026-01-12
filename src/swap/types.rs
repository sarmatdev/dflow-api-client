use serde::{Deserialize, Serialize};

// =============================================================================
// Common Types
// =============================================================================

/// Token information in swap context
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    /// Token mint address
    pub mint: String,
    /// Token symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Token decimals
    #[serde(default)]
    pub decimals: Option<u8>,
}

/// Price impact information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceImpact {
    /// Price impact percentage
    pub percent: f64,
    /// Price impact warning level (if applicable)
    #[serde(default)]
    pub warning: Option<String>,
}

/// Fee breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapFee {
    /// Fee amount in lamports or token units
    pub amount: String,
    /// Fee mint address
    #[serde(default)]
    pub mint: Option<String>,
    /// Fee percentage
    #[serde(default)]
    pub percent: Option<f64>,
}

// =============================================================================
// Imperative Swap API Types
// =============================================================================

/// Query parameters for GET /quote endpoint
#[derive(Debug, Clone, Default)]
pub struct GetQuoteParams {
    /// Input token mint address (required)
    pub input_mint: String,
    /// Output token mint address (required)
    pub output_mint: String,
    /// Amount to swap in smallest unit (e.g., lamports) (required)
    pub amount: String,
    /// Slippage tolerance in basis points (e.g., 50 = 0.5%)
    pub slippage_bps: Option<u32>,
    /// Whether the amount is for input (true) or output (false)
    pub exact_in: Option<bool>,
    /// User's wallet public key (optional, for priority fees)
    pub user_public_key: Option<String>,
}

/// Quote response from GET /quote endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    /// Input token mint address
    pub input_mint: String,
    /// Output token mint address
    pub output_mint: String,
    /// Input amount in smallest unit
    pub in_amount: String,
    /// Output amount in smallest unit
    pub out_amount: String,
    /// Minimum output amount after slippage
    #[serde(default)]
    pub other_amount_threshold: Option<String>,
    /// Swap mode (ExactIn or ExactOut)
    #[serde(default)]
    pub swap_mode: Option<String>,
    /// Slippage in basis points
    #[serde(default)]
    pub slippage_bps: Option<u32>,
    /// Price impact percentage
    #[serde(default)]
    pub price_impact_pct: Option<String>,
    /// Route information
    #[serde(default)]
    pub route_plan: Option<Vec<RoutePlanStep>>,
    /// Context slot for the quote
    #[serde(default)]
    pub context_slot: Option<u64>,
    /// Time taken for quote in milliseconds
    #[serde(default)]
    pub time_taken: Option<f64>,
}

/// A step in the route plan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanStep {
    /// AMM/DEX key
    #[serde(default)]
    pub amm_key: Option<String>,
    /// Label of the DEX (e.g., "Raydium", "Orca")
    #[serde(default)]
    pub label: Option<String>,
    /// Input mint for this step
    #[serde(default)]
    pub input_mint: Option<String>,
    /// Output mint for this step
    #[serde(default)]
    pub output_mint: Option<String>,
    /// Percentage of total input for this step
    #[serde(default)]
    pub percent: Option<u32>,
}

/// Request body for POST /swap endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SwapRequest {
    /// The quote response from GET /quote (required)
    pub quote_response: QuoteResponse,
    /// User's wallet public key (required)
    pub user_public_key: String,
    /// Wrap/unwrap SOL if needed
    #[serde(default)]
    pub wrap_and_unwrap_sol: Option<bool>,
    /// Use shared accounts to reduce transaction size
    #[serde(default)]
    pub use_shared_accounts: Option<bool>,
    /// Destination token account (if different from ATA)
    #[serde(default)]
    pub destination_token_account: Option<String>,
    /// Dynamic compute unit limit
    #[serde(default)]
    pub dynamic_compute_unit_limit: Option<bool>,
    /// Skip user accounts RPC calls
    #[serde(default)]
    pub skip_user_accounts_rpc_calls: Option<bool>,
    /// Priority fee configuration (in lamports or "auto")
    #[serde(default)]
    pub priority_fee: Option<serde_json::Value>,
}

/// Response from POST /swap endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    /// Base64-encoded serialized transaction
    pub swap_transaction: String,
    /// Last valid block height for the transaction
    #[serde(default)]
    pub last_valid_block_height: Option<u64>,
    /// Priority fee type used
    #[serde(default)]
    pub priority_fee_type: Option<String>,
    /// Priority fee lamports
    #[serde(default)]
    pub priority_fee_lamports: Option<u64>,
    /// Compute unit limit
    #[serde(default)]
    pub compute_unit_limit: Option<u32>,
    /// Dynamic slippage report
    #[serde(default)]
    pub dynamic_slippage_report: Option<serde_json::Value>,
    /// Simulation error if any
    #[serde(default)]
    pub simulation_error: Option<String>,
}

// =============================================================================
// Declarative Swap API Types
// =============================================================================

/// Query parameters for GET /intent endpoint
#[derive(Debug, Clone, Default)]
pub struct GetIntentParams {
    /// Input token mint address (required)
    pub input_mint: String,
    /// Output token mint address (required)
    pub output_mint: String,
    /// Amount to swap in smallest unit (required)
    pub amount: String,
    /// Whether the amount is for input (true) or output (false)
    pub exact_in: Option<bool>,
    /// User's wallet public key (optional)
    pub user_public_key: Option<String>,
}

/// Intent response from GET /intent endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentResponse {
    /// Input token mint address
    pub input_mint: String,
    /// Output token mint address
    pub output_mint: String,
    /// Input amount in smallest unit
    pub in_amount: String,
    /// Output amount in smallest unit
    pub out_amount: String,
    /// Intent ID for submission
    pub intent_id: String,
    /// Expiration timestamp for the intent (Unix timestamp in seconds)
    #[serde(default)]
    pub expires_at: Option<i64>,
    /// Price at quote time
    #[serde(default)]
    pub price: Option<String>,
    /// Swap mode
    #[serde(default)]
    pub swap_mode: Option<String>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Request body for POST /submit-intent endpoint
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubmitIntentRequest {
    /// Intent ID from GET /intent response (required)
    pub intent_id: String,
    /// User's wallet public key (required)
    pub user_public_key: String,
    /// Signed message or transaction (required)
    pub signature: String,
    /// Input token mint address
    #[serde(default)]
    pub input_mint: Option<String>,
    /// Output token mint address
    #[serde(default)]
    pub output_mint: Option<String>,
    /// Input amount
    #[serde(default)]
    pub in_amount: Option<String>,
    /// Minimum output amount
    #[serde(default)]
    pub min_out_amount: Option<String>,
}

/// Response from POST /submit-intent endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitIntentResponse {
    /// Submission status
    pub status: String,
    /// Intent ID
    pub intent_id: String,
    /// Transaction signature (if executed)
    #[serde(default)]
    pub transaction_signature: Option<String>,
    /// Expected output amount
    #[serde(default)]
    pub expected_out_amount: Option<String>,
    /// Estimated completion time in seconds
    #[serde(default)]
    pub estimated_completion_time: Option<u32>,
    /// Additional details or error information
    #[serde(default)]
    pub details: Option<String>,
}

/// Intent status for tracking submitted intents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntentStatus {
    Pending,
    Executing,
    Completed,
    Failed,
    Expired,
}

impl IntentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            IntentStatus::Pending => "pending",
            IntentStatus::Executing => "executing",
            IntentStatus::Completed => "completed",
            IntentStatus::Failed => "failed",
            IntentStatus::Expired => "expired",
        }
    }
}
