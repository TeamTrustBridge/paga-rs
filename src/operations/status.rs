//! Shared transaction status operations for the Paga Engine API.
//!
//! Covers the **Check the transaction status** operation that Paga documents
//! under money transfer, airtime and bill payment alike.

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
    /// Check what happened to a transaction you already sent.
    pub async fn get_transaction_status(
        &self,
        request: GetTransactionStatus,
    ) -> Result<TransactionStatusResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/transactionStatus",
            Some(&payload),
            RequestOptions::secured(TRANSACTION_STATUS_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }
}
