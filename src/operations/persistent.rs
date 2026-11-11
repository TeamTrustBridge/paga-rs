//! Persistent payment account operations for the Paga Engine API.
//!
//! A persistent payment account is a standing instruction that lets a
//! customer pay you repeatedly from the same account.

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
    /// Register a persistent payment account.
    pub async fn register_persistent_payment_account(
        &self,
        request: RegisterPersistentPaymentAccount,
    ) -> Result<PersistentPaymentAccountResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/registerPersistentPaymentAccount",
            Some(&payload),
            RequestOptions::basic(REGISTER_PERSISTENT_ACCOUNT_HASH_FIELDS),
        )
        .await
    }

    /// Delete a persistent payment account.
    pub async fn delete_persistent_payment_account(
        &self,
        reference_number: &str,
        account_identifier: &str,
    ) -> Result<PagaResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "accountIdentifier": account_identifier,
        });
        self.request(
            Method::POST,
            "/deletePersistentPaymentAccount",
            Some(&payload),
            RequestOptions::basic(DELETE_PERSISTENT_ACCOUNT_HASH_FIELDS),
        )
        .await
    }

    /// Update a persistent payment account.
    pub async fn update_persistent_payment_account(
        &self,
        request: UpdatePersistentPaymentAccount,
    ) -> Result<PersistentPaymentAccountResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/updatePersistentPaymentAccount",
            Some(&payload),
            RequestOptions::basic(UPDATE_PERSISTENT_ACCOUNT_HASH_FIELDS),
        )
        .await
    }

    /// Get a persistent payment account.
    pub async fn get_persistent_payment_account(
        &self,
        reference_number: &str,
        account_identifier: &str,
    ) -> Result<PersistentPaymentAccountResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "accountIdentifier": account_identifier,
        });
        self.request(
            Method::POST,
            "/getPersistentPaymentAccount",
            Some(&payload),
            RequestOptions::basic(GET_PERSISTENT_ACCOUNT_HASH_FIELDS),
        )
        .await
    }
}
