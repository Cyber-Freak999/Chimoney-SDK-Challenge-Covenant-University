use serde::{Deserialize, Serialize};

/// An AI agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub status: String,
    #[serde(default)]
    pub wallet_id: Option<String>,
    #[serde(default)]
    pub passport: Option<serde_json::Value>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to create an agent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgentRequest {
    /// Agent name.
    pub name: String,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Request to update an agent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAgentRequest {
    /// Agent ID to update.
    pub agent_id: String,
    /// Optional new name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional new description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Request to update agent policies.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAgentPoliciesRequest {
    /// Agent ID to update policies for.
    pub agent_id: String,
    /// Optional limits configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<serde_json::Value>,
    /// Optional capabilities configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    /// Optional regions configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<serde_json::Value>,
}

/// Request with just an agent ID.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentIdRequest {
    /// The agent ID.
    pub agent_id: String,
}

/// Response from getting or managing an agent API key.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentApiKeyResponse {
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub api_key_prefix: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to manage an agent API key.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManageAgentApiKeyRequest {
    /// The agent ID.
    pub agent_id: String,
    /// Action to perform: create, rotate, revoke, delete.
    pub action: String,
}

/// Request to get agent transactions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTransactionsRequest {
    /// The agent ID.
    pub agent_id: String,
    /// Optional sub-account filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from getting agent transactions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTransactionsResponse {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

/// Response from agent operations (single agent).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentResponse {
    pub status: String,
    #[serde(default)]
    pub data: Option<Agent>,
}

/// Response from listing agents.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentListResponse {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<Agent>>,
}

/// Request to fund an agent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundAgentRequest {
    /// The agent ID.
    pub agent_id: String,
    /// Amount in USD.
    pub amount_in_usd: f64,
    /// Optional sub-account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}
