use reqwest_middleware::ClientWithMiddleware;

use crate::error::{ChimoneyError, Result};
use crate::middleware::{build_client, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT_SECS};

/// Chimoney API client.
///
/// # Example
///
/// ```rust
/// use chimoney_rust_sdk::ChimoneyClient;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ChimoneyClient::new("your_api_key")?;
/// # Ok(())
/// # }
/// ```
pub struct ChimoneyClient {
    client: ClientWithMiddleware,
    api_key: String,
    base_url: String,
}

impl ChimoneyClient {
    /// Create a new ChimoneyClient with default settings.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key).build()
    }

    /// Create a new ChimoneyClient with sandbox URL.
    pub fn new_sandbox(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key)
            .base_url("https://api-v2-sandbox.chimoney.io")
            .build()
    }

    /// Get a builder for configuring the client.
    pub fn builder(api_key: impl Into<String>) -> ChimoneyClientBuilder {
        ChimoneyClientBuilder {
            api_key: api_key.into(),
            base_url: "https://api.chimoney.io".to_string(),
            max_retries: DEFAULT_MAX_RETRIES,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
        }
    }

    /// Get a reference to the HTTP client.
    pub fn http_client(&self) -> &ClientWithMiddleware {
        &self.client
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Make a GET request.
    async fn get(&self, path: &str, query: Option<&str>) -> Result<String> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(params) = query {
            url.push('?');
            url.push_str(params);
        }

        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-KEY", &self.api_key)
            .send()
            .await
            .map_err(ChimoneyError::MiddlewareError)?;

        self.handle_response(response).await
    }

    /// Make a POST request.
    async fn post(&self, path: &str, body: &str, query: Option<&str>) -> Result<String> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(params) = query {
            url.push('?');
            url.push_str(params);
        }

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-KEY", &self.api_key)
            .body(body.to_string())
            .send()
            .await
            .map_err(ChimoneyError::MiddlewareError)?;

        self.handle_response(response).await
    }

    /// Make a DELETE request.
    async fn delete(&self, path: &str, query: Option<&str>) -> Result<String> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(params) = query {
            url.push('?');
            url.push_str(params);
        }

        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-KEY", &self.api_key)
            .send()
            .await
            .map_err(ChimoneyError::MiddlewareError)?;

        self.handle_response(response).await
    }

    /// Make a PATCH request.
    async fn patch(&self, path: &str, body: &str, query: Option<&str>) -> Result<String> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(params) = query {
            url.push('?');
            url.push_str(params);
        }

        let response = self
            .client
            .patch(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-KEY", &self.api_key)
            .body(body.to_string())
            .send()
            .await
            .map_err(ChimoneyError::MiddlewareError)?;

        self.handle_response(response).await
    }

    // ── Account Methods ──────────────────────────────────────────────

    /// Get transactions by account ID.
    pub async fn get_transactions(&self, account_id: &str) -> Result<Vec<crate::types::Transaction>> {
        let path = "/v0.2.4/accounts/transactions";
        let body = serde_json::json!({ "subAccount": account_id }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Get single transaction details.
    pub async fn get_transaction(&self, transaction_id: &str) -> Result<crate::types::Transaction> {
        let path = "/v0.2.4/accounts/transaction";
        let body = serde_json::json!({ "id": transaction_id }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Get transaction by issue ID.
    pub async fn get_issue_id_transaction(
        &self,
        issue_id: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2.4/accounts/issue-id-transactions";
        let body = serde_json::json!({ "issueID": issue_id }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        Ok(json["data"].clone())
    }

    /// Get public profile.
    pub async fn get_public_profile(&self) -> Result<serde_json::Value> {
        let path = "/v0.2.4/accounts/public-profile";
        let body = "{}".to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        Ok(json["data"].clone())
    }

    /// Transfer between accounts.
    pub async fn transfer(
        &self,
        request: &crate::types::TransferRequest,
    ) -> Result<crate::types::TransferResponse> {
        let path = "/v0.2.4/accounts/transfer";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Initiate Chimoney transaction.
    pub async fn initiate_chimoney(
        &self,
        request: &crate::types::InitiateChimoneyRequest,
    ) -> Result<crate::types::InitiateChimoneyResponse> {
        let path = "/v0.2.4/payouts/initiate-chimoney";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Delete unpaid transactions.
    pub async fn delete_unpaid_transactions(
        &self,
        chi_ref: &str,
    ) -> Result<crate::types::DeleteUnpaidTransactionResponse> {
        let path = "/v0.2.4/accounts/delete-unpaid-transaction";
        let query = format!("chiRef={}", chi_ref);
        let response = self.delete(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    // ── Payment Methods ─────────────────────────────────────────────

    /// Initiate a payment.
    pub async fn initiate_payment(
        &self,
        request: &crate::types::PaymentRequest,
    ) -> Result<crate::types::PaymentResponse> {
        let path = "/v0.2.4/payment/initiate";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Verify a payment.
    pub async fn verify_payment(
        &self,
        issue_id: &str,
    ) -> Result<crate::types::PaymentVerification> {
        let path = "/v0.2.4/payment/verify";
        let body = serde_json::json!({ "issueID": issue_id }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Simulate a payment (sandbox only).
    pub async fn simulate_payment(
        &self,
        issue_id: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2.4/payment/simulate";
        let body = serde_json::json!({ "issueID": issue_id }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        Ok(json["data"].clone())
    }

    // ── Payout Methods ─────────────────────────────────────────────

    /// Payout via bank transfer.
    pub async fn payout_bank(
        &self,
        request: &crate::types::BankPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2.4/payouts/bank";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via airtime.
    pub async fn payout_airtime(
        &self,
        request: &crate::types::AirtimePayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2.4/payouts/airtime";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via Chimoney.
    pub async fn payout_chimoney(
        &self,
        request: &crate::types::ChimoneyPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2.4/payouts/chimoney";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via mobile money.
    pub async fn payout_mobile_money(
        &self,
        request: &crate::types::MobileMoneyPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2.4/payouts/mobile-money";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via gift card.
    pub async fn payout_giftcard(
        &self,
        request: &crate::types::GiftCardPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2.4/payouts/gift-card";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via Interledger wallet.
    pub async fn payout_interledger(
        &self,
        request: &crate::types::InterledgerPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2.4/payouts/interledger-wallet";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via wallet.
    pub async fn payout_wallet(
        &self,
        request: &crate::types::WalletPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2.4/payouts/wallet";
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Check payout status.
    pub async fn check_payout_status(
        &self,
        chi_ref: &str,
    ) -> Result<crate::types::PayoutStatusResponse> {
        let path = "/v0.2.4/payouts/status";
        let body = serde_json::json!({ "chiRef": chi_ref }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    // ── Redeem Methods ─────────────────────────────────────────────

    /// Redeem airtime.
    pub async fn redeem_airtime(
        &self,
        request: &crate::types::RedeemAirtimeRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2.4/redeem/airtime";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Redeem Chimoney.
    pub async fn redeem_chimoney(
        &self,
        request: &crate::types::RedeemChimoneyRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2.4/redeem/chimoney";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Redeem gift card.
    pub async fn redeem_giftcard(
        &self,
        request: &crate::types::RedeemGiftCardRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2.4/redeem/gift-card";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Redeem mobile money.
    pub async fn redeem_mobile_money(
        &self,
        request: &crate::types::RedeemMobileMoneyRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2.4/redeem/mobile-money";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    // ── SubAccount Methods ─────────────────────────────────────────

    /// Create a sub-account.
    pub async fn create_sub_account(
        &self,
        request: &crate::types::CreateSubAccountRequest,
    ) -> Result<crate::types::SubAccountResponse> {
        let path = "/v0.2.4/sub-account/create";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Update a sub-account.
    pub async fn update_sub_account(
        &self,
        request: &crate::types::UpdateSubAccountRequest,
    ) -> Result<crate::types::SubAccountResponse> {
        let path = "/v0.2.4/sub-account/update";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Delete a sub-account.
    pub async fn delete_sub_account(
        &self,
        sub_account_id: &str,
    ) -> Result<crate::types::SubAccountResponse> {
        let path = "/v0.2.4/sub-account/delete";
        let query = format!("id={}", sub_account_id);
        let response = self.delete(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Get sub-account details.
    pub async fn get_sub_account(
        &self,
        sub_account_id: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2.4/sub-account/get";
        let query = format!("id={}", sub_account_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// List all sub-accounts.
    pub async fn list_sub_accounts(
        &self,
    ) -> Result<Vec<serde_json::Value>> {
        let path = "/v0.2.4/sub-account/list";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    // ── Wallet Methods ─────────────────────────────────────────────

    /// List wallets.
    pub async fn list_wallets(
        &self,
        sub_account: &str,
    ) -> Result<crate::types::WalletList> {
        let path = "/v0.2.4/wallets/list";
        let body = serde_json::json!({ "subAccount": sub_account }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Lookup a wallet.
    pub async fn lookup_wallet(
        &self,
        request: &crate::types::WalletLookupRequest,
    ) -> Result<crate::types::WalletResponse> {
        let path = "/v0.2.4/wallets/lookup";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Transfer between wallets.
    pub async fn transfer_between_wallets(
        &self,
        request: &crate::types::WalletTransferRequest,
    ) -> Result<crate::types::WalletResponse> {
        let path = "/v0.2.4/wallets/transfer";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    // ── Info Methods ───────────────────────────────────────────────

    /// Get airtime countries.
    pub async fn get_airtime_countries(&self) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/airtime-countries";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get assets by country code.
    pub async fn get_assets(&self, country_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/assets";
        let query = format!("countryCode={}", country_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get banks by country code.
    pub async fn get_banks(&self, country_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/country-banks";
        let query = format!("countryCode={}", country_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get bank branches.
    pub async fn get_bank_branches(&self, bank_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/bank-branches";
        let query = format!("bankCode={}", bank_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get exchange rates.
    pub async fn get_exchange_rates(&self) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/exchange-rates";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Convert local currency to USD.
    pub async fn local_to_usd(
        &self,
        currency: &str,
        amount: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/local-amount-to-usd";
        let query = format!("originCurrency={}&amountInOriginCurrency={}", currency, amount);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Convert USD to local currency.
    pub async fn usd_to_local(
        &self,
        currency: &str,
        amount: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/usd-amount-in-local";
        let query = format!("destinationCurrency={}&amountInUSD={}", currency, amount);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get mobile money codes.
    pub async fn get_mobile_money_codes(&self) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/mobile-money-codes";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Verify bank account.
    pub async fn verify_bank_account(
        &self,
        country_code: &str,
        bank_code: &str,
        account_number: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/verify-bank-account";
        let body = serde_json::json!({
            "verifyAccountNumbers": [{
                "countryCode": country_code,
                "account_bank": bank_code,
                "account_number": account_number
            }]
        }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Handle API response.
    async fn handle_response(&self, response: reqwest::Response) -> Result<String> {
        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(ChimoneyError::RequestFailed)?;

        if status.is_success() {
            Ok(text)
        } else {
            let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|_| {
                serde_json::json!({
                    "code": status.as_u16(),
                    "message": text
                })
            });

            let message = json["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();

            if status.as_u16() == 429 {
                let retry_after = json["retry_after"].as_u64().unwrap_or(60);
                return Err(ChimoneyError::RateLimited { retry_after });
            }

            Err(ChimoneyError::ApiError {
                status: status.as_u16(),
                message,
            })
        }
    }
}

/// Builder for configuring `ChimoneyClient`.
pub struct ChimoneyClientBuilder {
    api_key: String,
    base_url: String,
    max_retries: u32,
    timeout_secs: u64,
}

impl ChimoneyClientBuilder {
    /// Set the base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the maximum number of retries.
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    /// Set the request timeout in seconds.
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<ChimoneyClient> {
        if self.api_key.is_empty() {
            return Err(ChimoneyError::ApiKeyEmpty);
        }

        let client = build_client(self.max_retries, self.timeout_secs)?;

        Ok(ChimoneyClient {
            client,
            api_key: self.api_key,
            base_url: self.base_url,
        })
    }
}
