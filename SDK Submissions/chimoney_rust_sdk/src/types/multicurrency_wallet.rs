use serde::{Deserialize, Serialize};

/// A multicurrency wallet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyWallet {
    pub id: String,
    pub currency: String,
    #[serde(default)]
    pub balance: Option<f64>,
    #[serde(default)]
    pub sub_account: Option<String>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to create a multicurrency wallet.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMulticurrencyWalletRequest {
    pub sub_account: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Request to update a multicurrency wallet.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMulticurrencyWalletRequest {
    pub wallet_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Response from multicurrency wallet operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyWalletResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<MulticurrencyWallet>,
}

/// Response from listing multicurrency wallets.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyWalletListResponse {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<MulticurrencyWallet>>,
}

/// Request to get a transfer quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferQuoteRequest {
    pub from_wallet: String,
    pub to_wallet: String,
    pub amount: f64,
    pub from_currency: String,
    pub to_currency: String,
}

/// Response from a transfer quote request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferQuoteResponse {
    pub status: String,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub source_amount: Option<f64>,
    #[serde(default)]
    pub destination_amount: Option<f64>,
}

/// Request to transfer between multicurrency wallets.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyTransferRequest {
    pub from_wallet: String,
    pub recipient: String,
    pub amount: f64,
    pub from_currency: String,
    pub to_currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Response from a multicurrency transfer.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyTransferResponse {
    pub status: String,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to issue a multicurrency wallet.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueWalletRequest {
    pub sub_account: String,
    pub currency: String,
}

/// Request to issue a bank account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueBankAccountRequest {
    pub sub_account: String,
    pub country_code: String,
    pub bank_code: String,
    pub account_number: String,
}
