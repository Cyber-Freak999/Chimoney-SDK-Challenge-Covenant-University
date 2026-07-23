use serde::{Deserialize, Serialize};

/// Request to initiate a payment.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequest {
    /// Payer's email address.
    pub email: String,
    /// Amount in USD.
    pub amount: f64,
    /// Redirect URL after payment.
    pub redirect_url: String,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from initiating a payment.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub checkout_url: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Payment verification result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentVerification {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}
