//! Refund operations for the Paga Engine API.
//!
//! Covers the **Accept payments → Refunds** section of Paga's documentation.

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
    /// Refund or cancel a payment request.
    pub async fn refund_payment(
        &self,
        request: RefundPayment,
    ) -> Result<RefundResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/refund/v2",
            Some(&payload),
            RequestOptions::basic(REFUND_HASH_FIELDS),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Actions                                                           //
    // ----------------------------------------------------------------- //

    /// Check whether a payment request was refunded.
    pub async fn get_refund_status(
        &self,
        reference_number: &str,
    ) -> Result<RefundStatusResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/status",
            Some(&payload),
            RequestOptions::basic(PAYMENT_STATUS_HASH_FIELDS).lenient(),
        )
        .await
    }
}
