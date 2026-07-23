use serde::{Deserialize, Serialize};

/// Base payout request with common fields.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutRequest {
    /// Sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Bank payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of bank transfers.
    pub transfers: Vec<BankTransfer>,
}

/// A single bank transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankTransfer {
    /// Bank code.
    pub bank_code: String,
    /// Account number.
    pub account_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Currency.
    pub currency: String,
    /// Country code.
    pub country_code: String,
    /// Beneficiary name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_name: Option<String>,
}

/// Airtime payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AirtimePayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of airtime transfers.
    pub transfers: Vec<AirtimeTransfer>,
}

/// A single airtime transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AirtimeTransfer {
    /// Phone number.
    pub phone_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Country code.
    pub country_code: String,
}

/// Chimoney payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChimoneyPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of Chimoney transfers.
    pub transfers: Vec<ChimoneyTransfer>,
}

/// A single Chimoney transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChimoneyTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
}

/// Mobile money payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileMoneyPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of mobile money transfers.
    pub transfers: Vec<MobileMoneyTransfer>,
}

/// A single mobile money transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileMoneyTransfer {
    /// Phone number.
    pub phone_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Country code.
    pub country_code: String,
    /// Mobile money provider code.
    pub provider_code: String,
}

/// Gift card payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftCardPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of gift card transfers.
    pub transfers: Vec<GiftCardTransfer>,
}

/// A single gift card transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftCardTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Gift card provider.
    pub provider: String,
}

/// Interledger wallet payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterledgerPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of Interledger transfers.
    pub transfers: Vec<InterledgerTransfer>,
}

/// A single Interledger transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterledgerTransfer {
    /// Receiver address.
    pub receiver_address: String,
    /// Amount in USD.
    pub value_in_usd: f64,
}

/// Wallet payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of wallet transfers.
    pub transfers: Vec<WalletTransfer>,
}

/// A single wallet transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Wallet ID.
    pub wallet_id: String,
}

/// Generic payout response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}

/// Payout status response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutStatusResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
