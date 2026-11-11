//! Business account operations for the Paga Engine API.
//!
//! Covers the **Manage accounts → Your business account** section of Paga's
//! documentation, plus the Business API operations that act on the account
//! itself.

#[allow(unused_imports)]
use crate::auth::*;
use crate::client::{PagaClient, RequestOptions};
use crate::errors::PagaError;
#[allow(unused_imports)]
use crate::requests::*;
#[allow(unused_imports)]
use crate::types::*;
use reqwest::Method;

impl PagaClient {
    /// Check the business account balance.
    pub async fn get_business_balance(
        &self,
        reference_number: &str,
    ) -> Result<AccountBalanceResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/accountBalance",
            Some(&payload),
            RequestOptions::secured(ACCOUNT_BALANCE_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Get the business transaction history.
    pub async fn get_business_transactions(
        &self,
        request: GetBusinessTransactions,
    ) -> Result<TransactionHistoryResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/transactionHistory",
            Some(&payload),
            RequestOptions::secured(TRANSACTION_HISTORY_HASH_FIELDS)
                .base_url(self.business_base_url.clone()),
        )
        .await
    }

    /// Get the five most recent transactions on the business account.
    pub async fn get_recent_transactions(
        &self,
        request: GetRecentTransactions,
    ) -> Result<RecentTransactionsResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/recentTransactionHistory",
            Some(&payload),
            RequestOptions::secured(RECENT_TRANSACTION_HISTORY_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Check an operation's status.
    pub async fn get_operation_status(
        &self,
        request: GetTransactionStatus,
    ) -> Result<OperationStatusResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/getOperationStatus",
            Some(&payload),
            RequestOptions::secured(GET_OPERATION_STATUS_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Onboard a merchant.
    pub async fn onboard_merchant(
        &self,
        request: OnboardMerchant,
    ) -> Result<OnboardMerchantResponse, PagaError> {
        let payload = request.to_payload();
        let hash_payload = request.to_hash_payload();
        self.request(
            Method::POST,
            "/onboardMerchant",
            Some(&payload),
            RequestOptions::secured(ONBOARD_MERCHANT_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .hash_payload(hash_payload)
                .lenient(),
        )
        .await
    }

    /// Dispense cash against a customer's withdrawal code.
    pub async fn dispense_cash(
        &self,
        request: DispenseCash,
    ) -> Result<DispenseCashResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/dispenseCash",
            Some(&payload),
            RequestOptions::secured(DISPENSE_CASH_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Subsidiary accounts                                               //
    // ----------------------------------------------------------------- //
}
