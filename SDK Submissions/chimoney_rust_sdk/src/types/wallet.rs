use serde::{Deserialize, Serialize};

/// Wallet details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub balance: Option<f64>,
}

/// List of wallets response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletList {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<Wallet>>,
}

/// Request to lookup a wallet.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletLookupRequest {
    /// Wallet ID.
    pub wallet_id: String,
    /// Sub-account ID.
    pub sub_account: String,
}

/// Request to transfer between wallets.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletTransferRequest {
    /// Source wallet ID.
    pub wallet: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Sub-account ID.
    pub sub_account: String,
    /// Receiver email or ID.
    pub receiver: String,
}

/// Response from wallet operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
