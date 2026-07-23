use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Base redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemRequest {
    /// Sub-account ID.
    pub sub_account: String,
    /// Chi reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chi_ref: Option<String>,
    /// Turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Airtime redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemAirtimeRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Country code.
    pub country_to_send: String,
    /// Phone number.
    pub phone_number: String,
    /// Test mode flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

/// Chimoney redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemChimoneyRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Chimoney key-value pairs.
    pub chimoneys: HashMap<String, String>,
}

/// Gift card redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemGiftCardRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Redeem options key-value pairs.
    pub redeem_options: HashMap<String, String>,
}

/// Mobile money redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemMobileMoneyRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Redeem options key-value pairs.
    pub redeem_options: HashMap<String, String>,
}

/// Generic redeem response.
#[derive(Debug, Clone, Deserialize)]
pub struct RedeemResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}
