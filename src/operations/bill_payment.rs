//! Bill payment operations for the Paga Engine API.
//!
//! Covers the **Make payments → Bill payment (VAS)** section of Paga's
//! documentation.

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
    /// List every biller you can pay.
    pub async fn get_merchants(&self, reference_number: &str) -> Result<MerchantsResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/getMerchants",
            Some(&payload),
            RequestOptions::secured(MERCHANTS_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// List what a biller can be paid for.
    pub async fn get_merchant_services(
        &self,
        reference_number: &str,
        merchant_public_id: &str,
    ) -> Result<MerchantServicesResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "merchantPublicId": merchant_public_id,
        });
        self.request(
            Method::POST,
            "/getMerchantServices",
            Some(&payload),
            RequestOptions::secured(MERCHANT_SERVICES_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Validate a customer's meter, smart card or account number.
    ///
    /// `merchant_service_product_code` takes part in the hash even when the
    /// customer chose no service.
    pub async fn get_merchant_account_details(
        &self,
        reference_number: &str,
        merchant_account: &str,
        merchant_reference_number: &str,
        merchant_service_product_code: Option<&str>,
    ) -> Result<MerchantAccountDetailsResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "merchantAccount": merchant_account,
            "merchantReferenceNumber": merchant_reference_number,
            "merchantServiceProductCode": merchant_service_product_code.unwrap_or(""),
        });
        self.request(
            Method::POST,
            "/getMerchantAccountDetails",
            Some(&payload),
            RequestOptions::secured(MERCHANT_ACCOUNT_DETAILS_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Pay a bill from the business balance.
    pub async fn pay_bill(&self, request: PayBill) -> Result<MerchantPaymentResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/merchantPayment",
            Some(&payload),
            RequestOptions::secured(BILL_PAYMENT_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Business account                                                  //
    // ----------------------------------------------------------------- //
}
