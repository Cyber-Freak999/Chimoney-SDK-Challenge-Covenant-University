# Task 6: Add Account Methods

**Files:**
- Modify: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyClient`, account types
- Produces: Account methods on `ChimoneyClient`

## Steps

- [ ] **Step 1: Add account methods to client.rs**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Get transactions by account ID.
    pub async fn get_transactions(&self, account_id: &str) -> Result<Vec<crate::types::Transaction>> {
        let path = "/v0.2/accounts/transactions";
        let query = format!("accountId={}", account_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Get single transaction details.
    pub async fn get_transaction(&self, transaction_id: &str) -> Result<crate::types::Transaction> {
        let path = "/v0.2/accounts/transaction";
        let query = format!("id={}", transaction_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Get transaction by issue ID.
    pub async fn get_issue_id_transaction(&self, issue_id: &str) -> Result<serde_json::Value> {
        let path = "/v0.2/accounts/issue-id-transactions";
        let query = format!("issueId={}", issue_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get public profile.
    pub async fn get_public_profile(&self) -> Result<serde_json::Value> {
        let path = "/v0.2/accounts/public-profile";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Transfer between accounts.
    pub async fn transfer(
        &self,
        request: &crate::types::TransferRequest,
    ) -> Result<crate::types::TransferResponse> {
        let path = "/v0.2/accounts/transfer";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Initiate Chimoney transaction.
    pub async fn initiate_chimoney(
        &self,
        request: &crate::types::InitiateChimoneyRequest,
    ) -> Result<crate::types::InitiateChimoneyResponse> {
        let path = "/v0.2/payouts/initiate-chimoney";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Delete unpaid transactions.
    pub async fn delete_unpaid_transactions(
        &self,
        chi_ref: &str,
    ) -> Result<crate::types::DeleteUnpaidTransactionResponse> {
        let path = "/v0.2/accounts/delete-unpaid-transaction";
        let query = format!("chiRef={}", chi_ref);
        let response = self.delete(path, Some(&query)).await?;
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
git commit -m "feat: add account methods to ChimoneyClient"
```

## Notes

- This task adds account-related methods to the ChimoneyClient
- Methods include: get_transactions, get_transaction, get_issue_id_transaction, get_public_profile, transfer, initiate_chimoney, delete_unpaid_transactions
- All methods return typed responses using the types defined in Task 3
- Methods use the internal `get`, `post`, `delete` methods from Task 5
- This builds on the foundation tasks (error types, types, client struct)
