//! Airtime and data purchase operations for the Paga Engine API.
//!
//! Covers the **Make payments → Airtime and data** section of Paga's
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
use serde_json::Value;

impl PagaClient {
    /// List the mobile networks you can top up.
    pub async fn get_mobile_operators(
        &self,
        reference_number: &str,
        locale: Option<&str>,
    ) -> Result<MobileOperatorsResponse, PagaError> {
        let mut payload = serde_json::Map::new();
        payload.insert("referenceNumber".into(), serde_json::json!(reference_number));
        if let Some(locale) = locale {
            payload.insert("locale".into(), serde_json::json!(locale));
        }
        self.request(
            Method::POST,
            "/getMobileOperators",
            Some(&Value::Object(payload)),
            RequestOptions::secured(MOBILE_OPERATORS_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// List a network's data bundles.
    pub async fn get_data_bundles(
        &self,
        reference_number: &str,
        operator_public_id: &str,
    ) -> Result<DataBundlesResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "operatorPublicId": operator_public_id,
        });
        self.request(
            Method::POST,
            "/getDataBundleByOperator",
            Some(&payload),
            RequestOptions::secured(DATA_BUNDLE_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Buy airtime or data.
    pub async fn buy_airtime(&self, request: BuyAirtime) -> Result<PurchaseResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/airtimePurchase",
            Some(&payload),
            RequestOptions::secured(AIRTIME_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }
}
