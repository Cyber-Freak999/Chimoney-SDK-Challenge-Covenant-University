use serde::{Deserialize, Serialize};

/// A beneficiary for payouts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beneficiary {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub beneficiary_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bank_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub country_code: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
}

/// Request to create a bank beneficiary.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBankBeneficiaryRequest {
    pub account_number: String,
    pub bank_code: String,
    pub country_code: String,
    pub name: String,
    pub currency: String,
}

/// Response from creating or getting a beneficiary.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeneficiaryResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<Beneficiary>,
}

/// Response from listing beneficiaries.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeneficiaryListResponse {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<Beneficiary>>,
}

/// Request to preview a transfer to a beneficiary.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewTransferRequest {
    pub beneficiary_id: String,
    pub amount: f64,
    pub currency: String,
}

/// Response from previewing a transfer.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewTransferResponse {
    pub status: String,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub total_amount: Option<f64>,
    #[serde(default)]
    pub destination_amount: Option<f64>,
}
