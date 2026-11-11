//! Subsidiary account operations for the Paga Engine API.
//!
//! Covers the **Manage accounts → Subsidiary accounts** section of Paga's
//! documentation.
//!
//! A subsidiary account is a Paga account for one of your customers, held on
//! your behalf. You can issue, hold and move money on their account, and set
//! it up to sweep straight into a bank account.

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
    /// Create a subsidiary account.
    pub async fn create_subsidiary_account(
        &self,
        request: CreateSubsidiaryAccount,
    ) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/subsidiary-accounts",
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_ACCOUNT_HASH_FIELDS),
        )
        .await
    }

    /// Charge a ledger-backed subsidiary account.
    pub async fn charge_subsidiary_account(
        &self,
        request: ChargeSubsidiaryAccount,
    ) -> Result<SubsidiaryMoneyResponse, PagaError> {
        let payload = request.to_payload();
        let path = format!(
            "/subsidiary-accounts/{}/charge",
            request.account_reference
        );
        self.request(
            Method::POST,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_CHARGE_HASH_FIELDS),
        )
        .await
    }

    /// Top up a ledger-backed subsidiary account.
    pub async fn top_up_subsidiary_account(
        &self,
        request: TopUpSubsidiaryAccount,
    ) -> Result<SubsidiaryMoneyResponse, PagaError> {
        let payload = request.to_payload();
        let path = format!("/subsidiary-accounts/{}/topup", request.account_reference);
        self.request(
            Method::POST,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_TOPUP_HASH_FIELDS),
        )
        .await
    }

    /// Transfer money between two subsidiary accounts.
    pub async fn transfer_between_subsidiary_accounts(
        &self,
        request: TransferBetweenSubsidiaryAccounts,
    ) -> Result<SubsidiaryTransferResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/subsidiary-accounts/transfer",
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_TRANSFER_HASH_FIELDS),
        )
        .await
    }

    /// Retrieve a subsidiary account.
    pub async fn retrieve_subsidiary_account(
        &self,
        account_reference: &str,
        reference_number: &str,
    ) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "accountIdentifier": account_reference,
        });
        let path = format!("/subsidiary-accounts/{}", account_reference);
        self.request(
            Method::GET,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_RETRIEVE_HASH_FIELDS)
                .base_url(self.base_url.clone())
                .param("referenceNumber", reference_number),
        )
        .await
    }

    /// Update a subsidiary account.
    pub async fn update_subsidiary_account(
        &self,
        request: UpdateSubsidiaryAccount,
    ) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = request.to_payload();
        let path = format!("/subsidiary-accounts/{}", request.account_reference);
        self.request(
            Method::PUT,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_UPDATE_HASH_FIELDS),
        )
        .await
    }

    /// Get a subsidiary account balance.
    pub async fn get_subsidiary_balance(
        &self,
        account_reference: &str,
        reference_number: &str,
    ) -> Result<SubsidiaryBalanceResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "accountIdentifier": account_reference,
        });
        let path = format!("/subsidiary-accounts/{}/balance", account_reference);
        self.request(
            Method::GET,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_BALANCE_HASH_FIELDS)
                .base_url(self.base_url.clone())
                .param("referenceNumber", reference_number),
        )
        .await
    }

    /// Get a subsidiary account transaction history.
    pub async fn get_subsidiary_history(
        &self,
        request: GetSubsidiaryHistory,
    ) -> Result<SubsidiaryHistoryResponse, PagaError> {
        let payload = request.to_payload();
        let path = format!(
            "/subsidiary-accounts/{}/history",
            request.account_reference
        );
        self.request(
            Method::GET,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_HISTORY_HASH_FIELDS)
                .base_url(self.base_url.clone())
                .param("referenceNumber", request.reference_number.clone())
                .param("startDateTimeUTC", request.start_date_time_utc.clone())
                .param("endDateTimeUTC", request.end_date_time_utc.clone())
                .param("pageSize", request.page_size.unwrap_or(20).to_string())
                .param("pageNumber", request.page_number.unwrap_or(0).to_string()),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Directory lookups                                                 //
    // ----------------------------------------------------------------- //
}
