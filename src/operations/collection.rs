//! Payment collection operations for the Paga Engine API.
//!
//! Covers the **Accept payments** sections of Paga's documentation that
//! collect money from a customer:
//!
//! - **Dynamic virtual account** — create a payment request, check its status.
//! - **Bank USSD** — list the banks a customer can pay from.
//! - **Paga wallet** — the same payment request, with `paymentMethods` set to
//!   `["PAGA_WALLET"]`.
//! - **Bank transfer (ACH, Wire and Swift)** — the same payment request in
//!   USD, plus its own status check.
//!
//! The payment request itself is shared by all four, so it lives here once.

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
    /// Create a payment request.
    pub async fn create_payment(
        &self,
        request: CreatePayment,
    ) -> Result<CollectPaymentRequestResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/paymentRequest",
            Some(&payload),
            RequestOptions::basic(PAYMENT_REQUEST_HASH_FIELDS),
        )
        .await
    }

    /// Check the status of a payment request.
    pub async fn get_payment_status(
        &self,
        reference_number: &str,
    ) -> Result<PaymentStatusResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/status",
            Some(&payload),
            RequestOptions::basic(PAYMENT_STATUS_HASH_FIELDS),
        )
        .await
    }

    /// List the banks a customer can pay from.
    pub async fn get_collect_banks(
        &self,
        reference_number: &str,
    ) -> Result<CollectBanksResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/banks",
            Some(&payload),
            RequestOptions::basic(COLLECT_BANKS_HASH_FIELDS).lenient(),
        )
        .await
    }

    /// Check the status of a USD (ACH, Wire, Swift) payment request.
    pub async fn get_usd_payment_status(
        &self,
        reference_number: &str,
        payment_currency: &str,
    ) -> Result<PagaResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "paymentCurrency": payment_currency,
        });
        self.request(
            Method::POST,
            "/status",
            Some(&payload),
            RequestOptions::basic(USD_PAYMENT_STATUS_HASH_FIELDS).lenient(),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Direct Debit                                                      //
    // ----------------------------------------------------------------- //

    /// Get the history of your payment requests.
    pub async fn get_payment_request_history(
        &self,
        reference_number: &str,
    ) -> Result<PagaResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/history",
            Some(&payload),
            RequestOptions::basic(HISTORY_HASH_FIELDS).lenient(),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Customers                                                         //
    // ----------------------------------------------------------------- //
}
