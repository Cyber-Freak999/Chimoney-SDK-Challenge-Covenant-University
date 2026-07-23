# Task 3: Create Request/Response Types

**Files:**
- Create: `src/types/mod.rs`
- Create: `src/types/account.rs`
- Create: `src/types/payment.rs`
- Create: `src/types/payout.rs`
- Create: `src/types/redeem.rs`
- Create: `src/types/subaccount.rs`
- Create: `src/types/wallet.rs`

**Interfaces:**
- Consumes: None
- Produces: Typed request/response structs for all API endpoints

## Steps

- [ ] **Step 1: Create src/types/mod.rs**

```rust
mod account;
mod payment;
mod payout;
mod redeem;
mod subaccount;
mod wallet;

pub use account::*;
pub use payment::*;
pub use payout::*;
pub use redeem::*;
pub use subaccount::*;
pub use wallet::*;
```

- [ ] **Step 2: Create src/types/account.rs**

```rust
use serde::{Deserialize, Serialize};

/// A Chimoney transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to transfer between accounts.
#[derive(Debug, Clone, Serialize)]
pub struct TransferRequest {
    /// The receiver's ID or email.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from a transfer request.
#[derive(Debug, Clone, Deserialize)]
pub struct TransferResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to initiate a Chimoney transaction.
#[derive(Debug, Clone, Serialize)]
pub struct InitiateChimoneyRequest {
    /// Receiver's email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Optional turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Response from initiating a Chimoney transaction.
#[derive(Debug, Clone, Deserialize)]
pub struct InitiateChimoneyResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to delete unpaid transactions.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteUnpaidTransactionRequest {
    /// The chi reference to delete.
    pub chi_ref: String,
}

/// Response from deleting unpaid transactions.
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUnpaidTransactionResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}
```

- [ ] **Step 3: Create src/types/payment.rs**

```rust
use serde::{Deserialize, Serialize};

/// Request to initiate a payment.
#[derive(Debug, Clone, Serialize)]
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
```

- [ ] **Step 4: Create src/types/payout.rs**

```rust
use serde::{Deserialize, Serialize};

/// Base payout request with common fields.
#[derive(Debug, Clone, Serialize)]
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
pub struct BankPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of bank transfers.
    pub transfers: Vec<BankTransfer>,
}

/// A single bank transfer.
#[derive(Debug, Clone, Serialize)]
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
pub struct AirtimePayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of airtime transfers.
    pub transfers: Vec<AirtimeTransfer>,
}

/// A single airtime transfer.
#[derive(Debug, Clone, Serialize)]
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
pub struct ChimoneyPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of Chimoney transfers.
    pub transfers: Vec<ChimoneyTransfer>,
}

/// A single Chimoney transfer.
#[derive(Debug, Clone, Serialize)]
pub struct ChimoneyTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
}

/// Mobile money payout request.
#[derive(Debug, Clone, Serialize)]
pub struct MobileMoneyPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of mobile money transfers.
    pub transfers: Vec<MobileMoneyTransfer>,
}

/// A single mobile money transfer.
#[derive(Debug, Clone, Serialize)]
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
pub struct GiftCardPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of gift card transfers.
    pub transfers: Vec<GiftCardTransfer>,
}

/// A single gift card transfer.
#[derive(Debug, Clone, Serialize)]
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
pub struct InterledgerPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of Interledger transfers.
    pub transfers: Vec<InterledgerTransfer>,
}

/// A single Interledger transfer.
#[derive(Debug, Clone, Serialize)]
pub struct InterledgerTransfer {
    /// Receiver address.
    pub receiver_address: String,
    /// Amount in USD.
    pub value_in_usd: f64,
}

/// Wallet payout request.
#[derive(Debug, Clone, Serialize)]
pub struct WalletPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of wallet transfers.
    pub transfers: Vec<WalletTransfer>,
}

/// A single wallet transfer.
#[derive(Debug, Clone, Serialize)]
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
pub struct PayoutResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}

/// Payout status response.
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutStatusResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
```

- [ ] **Step 5: Create src/types/redeem.rs**

```rust
use serde::{Deserialize, Serialize};

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
    pub chimoneys: std::collections::HashMap<String, String>,
}

/// Gift card redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemGiftCardRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Redeem options key-value pairs.
    pub redeem_options: std::collections::HashMap<String, String>,
}

/// Mobile money redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemMobileMoneyRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Redeem options key-value pairs.
    pub redeem_options: std::collections::HashMap<String, String>,
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
```

- [ ] **Step 6: Create src/types/subaccount.rs**

```rust
use serde::{Deserialize, Serialize};

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
    pub meta: Option<std::collections::HashMap<String, String>>,
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
```

- [ ] **Step 7: Create src/types/wallet.rs**

```rust
use serde::{Deserialize, Serialize};

/// Wallet details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub balance: Option<f64>,
}

/// List of wallets response.
#[derive(Debug, Clone, Deserialize)]
pub struct WalletList {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<Wallet>>,
}

/// Request to lookup a wallet.
#[derive(Debug, Clone, Serialize)]
pub struct WalletLookupRequest {
    /// Wallet ID.
    pub wallet_id: String,
    /// Sub-account ID.
    pub sub_account: String,
}

/// Request to transfer between wallets.
#[derive(Debug, Clone, Serialize)]
pub struct WalletTransferRequest {
    /// Source wallet ID.
    pub wallet: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Sub-account ID.
    pub sub_account: String,
    /// Receiver email or ID.
    pub receiver: String,
}

/// Response from wallet operations.
#[derive(Debug, Clone, Deserialize)]
pub struct WalletResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
```

- [ ] **Step 8: Run cargo check to verify types compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 9: Commit**

```bash
git add src/types/
git commit -m "feat: add typed request/response structs"
```

## Notes

- This task creates all the typed request/response structs for the SDK
- All structs use serde for serialization/deserialization
- Optional fields use `Option<T>` with `skip_serializing_if` for clean JSON
- Request structs use `#[derive(Serialize)]`
- Response structs use `#[derive(Deserialize)]`
- Some structs use `#[serde(flatten)]` for composition (e.g., `BankPayoutRequest` flattens `PayoutRequest`)
- This is a foundation task - other tasks depend on these types
