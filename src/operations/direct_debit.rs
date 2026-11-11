//! Direct Debit (Open Banking) operations for the Paga Engine API.
//!
//! Covers the **Accept payments → Direct debit (Open banking)** section of
//! Paga's documentation.
//!
//! The flow is: start a hosted set-up session (or create a mandate by API),
//! wait for the mandate callback, then charge the mandate. Only charge a
//! mandate whose `ProcessStatusId` is `APPROVED`.

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
    /// Start a hosted Direct Debit set-up session.
    ///
    /// This endpoint uses HTTP Basic only — it sends no `hash` header.
    pub async fn start_direct_debit_session(
        &self,
        request: StartDirectDebitSession,
    ) -> Result<DirectDebitSessionResponse, PagaError> {
        let payload = request.to_payload();
        let options = RequestOptions::basic_without_hash()
            .base_url(self.checkout_base_url.clone())
            .lenient();
        self.request(
            Method::POST,
            "/checkout/charge-process/start",
            Some(&payload),
            options,
        )
        .await
    }

    /// Set up a Direct Debit mandate by API.
    pub async fn create_direct_debit_mandate(
        &self,
        request: CreateDirectDebitMandate,
    ) -> Result<PagaResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/paymentRequest",
            Some(&payload),
            RequestOptions::basic(PAYMENT_REQUEST_HASH_FIELDS),
        )
        .await
    }

    /// Check a mandate's status.
    pub async fn get_mandate_status(
        &self,
        reference_number: &str,
    ) -> Result<MandateStatusResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/status",
            Some(&payload),
            RequestOptions::basic(MANDATE_STATUS_HASH_FIELDS).lenient(),
        )
        .await
    }

    /// Charge an approved Direct Debit mandate.
    pub async fn charge_mandate(
        &self,
        request: ChargeMandate,
    ) -> Result<ChargeMandateResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/chargeDebitMandate",
            Some(&payload),
            RequestOptions::basic(CHARGE_MANDATE_HASH_FIELDS),
        )
        .await
    }

    /// Check a mandate charge's status.
    pub async fn get_charge_mandate_status(
        &self,
        reference_number: &str,
    ) -> Result<ChargeMandateStatusResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/getChargeMandateStatus",
            Some(&payload),
            RequestOptions::basic(CHARGE_MANDATE_STATUS_HASH_FIELDS).lenient(),
        )
        .await
    }

    /// Disable a Direct Debit mandate.
    pub async fn disable_mandate(
        &self,
        request: DisableMandate,
    ) -> Result<DisableMandateResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/disableMandate",
            Some(&payload),
            RequestOptions::basic(DISABLE_MANDATE_HASH_FIELDS),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Persistent payment accounts                                       //
    // ----------------------------------------------------------------- //
}
