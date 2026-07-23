use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sub-account details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccount {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
}

/// Request to create a sub-account.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubAccountRequest {
    /// Sub-account name.
    pub name: String,
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Email address.
    pub email: String,
    /// Phone number.
    pub phone_number: String,
}

/// Request to update a sub-account.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSubAccountRequest {
    /// Sub-account ID.
    pub id: String,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Metadata key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, String>>,
}

/// Response from sub-account operations.
#[derive(Debug, Clone, Deserialize)]
pub struct SubAccountResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// List of sub-accounts.
#[derive(Debug, Clone, Deserialize)]
pub struct SubAccountList {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<SubAccount>>,
}
