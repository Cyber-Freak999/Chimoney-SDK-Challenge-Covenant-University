use serde::{Deserialize, Serialize};

/// A Chimoney transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to transfer between accounts.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    /// The receiver's ID or email.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from a transfer request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to initiate a Chimoney transaction.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiateChimoneyRequest {
    /// Receiver's email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Optional turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Response from initiating a Chimoney transaction.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiateChimoneyResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to delete unpaid transactions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUnpaidTransactionRequest {
    /// The chi reference to delete.
    pub chi_ref: String,
}

/// Response from deleting unpaid transactions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUnpaidTransactionResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}
