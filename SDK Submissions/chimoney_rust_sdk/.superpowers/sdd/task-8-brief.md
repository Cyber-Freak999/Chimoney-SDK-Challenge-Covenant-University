# Task 8: Add Payout Methods

**Files:**
- Modify: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyClient`, payout types
- Produces: Payout methods on `ChimoneyClient`

## Steps

- [ ] **Step 1: Add payout methods to client.rs**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Payout via bank transfer.
    pub async fn payout_bank(
        &self,
        request: &crate::types::BankPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/bank";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via airtime.
    pub async fn payout_airtime(
        &self,
        request: &crate::types::AirtimePayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/airtime";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via Chimoney.
    pub async fn payout_chimoney(
        &self,
        request: &crate::types::ChimoneyPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/chimoney";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via mobile money.
    pub async fn payout_mobile_money(
        &self,
        request: &crate::types::MobileMoneyPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/mobile-money";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via gift card.
    pub async fn payout_giftcard(
        &self,
        request: &crate::types::GiftCardPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/gift-card";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via Interledger wallet.
    pub async fn payout_interledger(
        &self,
        request: &crate::types::InterledgerPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/interledger-wallet";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via wallet.
    pub async fn payout_wallet(
        &self,
        request: &crate::types::WalletPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/wallet";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Check payout status.
    pub async fn check_payout_status(
        &self,
        chi_ref: &str,
    ) -> Result<crate::types::PayoutStatusResponse> {
        let path = "/v0.2/payouts/status";
        let query = format!("chiRef={}", chi_ref);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 2: Run cargo check to verify methods compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/client.rs
git commit -m "feat: add payout methods to ChimoneyClient"
```

## Notes

- This task adds payout-related methods to the ChimoneyClient
- Methods include: payout_bank, payout_airtime, payout_chimoney, payout_mobile_money, payout_giftcard, payout_interledger, payout_wallet, check_payout_status
- All methods return typed responses using the types defined in Task 3
- This covers the core payout functionality of the SDK
- This builds on the foundation tasks (error types, types, client struct)
