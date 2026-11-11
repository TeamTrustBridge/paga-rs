//! Checkout operations for the Paga Engine API.
//!
//! Covers the **Accept payments → Checkout** section of Paga's documentation.
//!
//! Checkout takes a card or bank payment on a Paga-hosted page. The customer
//! redirects to Paga, pays, and Paga notifies your server.

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
    /// Verify a checkout charge.
    ///
    /// `status_code` `0` means paid, `2` means pending. The response is
    /// returned rather than raised: a pending charge is a legitimate answer.
    pub async fn verify_charge(
        &self,
        payment_reference: &str,
        public_key: &str,
        amount: f64,
    ) -> Result<CheckoutVerifyChargeResponse, PagaError> {
        let payload = serde_json::json!({
            "paymentReference": payment_reference,
            "publicKey": public_key,
            "amount": amount,
            "currency": "NGN",
        });
        let options = RequestOptions::basic_without_hash()
            .base_url(self.checkout_base_url.clone())
            .lenient();
        self.request(
            Method::POST,
            "/checkout/transaction/verify",
            Some(&payload),
            options,
        )
        .await
    }
}
