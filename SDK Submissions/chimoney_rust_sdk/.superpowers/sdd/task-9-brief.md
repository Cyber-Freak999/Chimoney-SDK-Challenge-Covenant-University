# Task 9: Add Redeem, SubAccount, Wallet, Info Methods

**Files:**
- Modify: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyClient`, all remaining types
- Produces: All remaining methods on `ChimoneyClient`

## Steps

- [ ] **Step 1: Add redeem methods**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Redeem airtime.
    pub async fn redeem_airtime(
        &self,
        request: &crate::types::RedeemAirtimeRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2/redeem/airtime";
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
        let path = "/v0.2/redeem/chimoney";
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
        let path = "/v0.2/redeem/gift-card";
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
        let path = "/v0.2/redeem/mobile-money";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 2: Add subaccount methods**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Create a sub-account.
    pub async fn create_sub_account(
        &self,
        request: &crate::types::CreateSubAccountRequest,
    ) -> Result<crate::types::SubAccountResponse> {
        let path = "/v0.2/sub-account/create";
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
        let path = "/v0.2/sub-account/update";
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
        let path = "/v0.2/sub-account/delete";
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
        let path = "/v0.2/sub-account/get";
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
        let path = "/v0.2/sub-account/list";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 3: Add wallet methods**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// List wallets.
    pub async fn list_wallets(
        &self,
        sub_account: &str,
    ) -> Result<crate::types::WalletList> {
        let path = "/v0.2/wallets/list";
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
        let path = "/v0.2/wallets/lookup";
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
        let path = "/v0.2/wallets/transfer";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 4: Add info methods**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Get airtime countries.
    pub async fn get_airtime_countries(&self) -> Result<serde_json::Value> {
        let path = "/v0.2/info/airtime-countries";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get assets by country code.
    pub async fn get_assets(&self, country_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2/info/assets";
        let query = format!("countryCode={}", country_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get banks by country code.
    pub async fn get_banks(&self, country_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2/info/country-banks";
        let query = format!("countryCode={}", country_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get bank branches.
    pub async fn get_bank_branches(&self, bank_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2/info/bank-branches";
        let query = format!("bankCode={}", bank_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get exchange rates.
    pub async fn get_exchange_rates(&self) -> Result<serde_json::Value> {
        let path = "/v0.2/info/exchange-rates";
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
        let path = "/v0.2/info/local-amount-to-usd";
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
        let path = "/v0.2/info/usd-amount-in-local";
        let query = format!("destinationCurrency={}&amountInUSD={}", currency, amount);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get mobile money codes.
    pub async fn get_mobile_money_codes(&self) -> Result<serde_json::Value> {
        let path = "/v0.2/info/mobile-money-codes";
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
        let path = "/v0.2/info/verify-bank-account";
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
}
```

- [ ] **Step 5: Run cargo check to verify all methods compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 6: Commit**

```bash
git add src/client.rs
git commit -m "feat: add redeem, subaccount, wallet, and info methods"
```

## Notes

- This task adds all remaining methods to the ChimoneyClient
- Includes: redeem (4 methods), subaccount (6 methods), wallet (3 methods), info (9 methods)
- All methods return typed responses using the types defined in Task 3
- This completes the API method implementation
- This builds on the foundation tasks (error types, types, client struct)
