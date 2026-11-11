//! Customer registration and KYC operations for the Paga Engine API.
//!
//! Registering a customer on the Business API is how you open an account for
//! someone. The plain registration takes the customer's details; the KYC
//! variants additionally take photographs, and are sent as
//! `multipart/form-data`.

#[allow(unused_imports)]
use crate::auth::*;
use crate::client::{PagaClient, RequestOptions};
use crate::client::{part_bytes, part_text};
use crate::errors::PagaError;
#[allow(unused_imports)]
use crate::requests::*;
#[allow(unused_imports)]
use crate::types::*;
use reqwest::Method;
use reqwest::multipart::Form;

impl PagaClient {
    /// Register a customer.
    pub async fn register_customer(
        &self,
        request: RegisterCustomer,
    ) -> Result<CustomerResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/registerCustomer",
            Some(&payload),
            RequestOptions::secured(CUSTOMER_HASH_FIELDS)
                .base_url(self.business_base_url.clone()),
        )
        .await
    }

    /// Register a customer with KYC photographs.
    pub async fn register_customer_with_kyc(
        &self,
        request: RegisterCustomerWithKyc,
    ) -> Result<CustomerResponse, PagaError> {
        let payload = request.to_payload();
        let form = Form::new()
            .part(
                "customer",
                part_text(payload.to_string(), "application/json")?,
            )
            .part(
                "customerAccountPhoto",
                part_bytes(request.customer_account_photo.clone(), "account-photo.jpg", "image/jpeg")?,
            )
            .part(
                "customerIdPhoto",
                part_bytes(request.customer_id_photo.clone(), "id-photo.jpg", "image/jpeg")?,
            );
        self.request_multipart(
            "/registerCustomerWithKYC",
            form,
            Some(&payload),
            CUSTOMER_HASH_FIELDS,
            true,
        )
        .await
    }

    /// Upload a customer's account photograph.
    pub async fn register_customer_account_photo(
        &self,
        request: RegisterCustomerAccountPhoto,
    ) -> Result<CustomerResponse, PagaError> {
        let payload = request.to_payload();
        let form = Form::new()
            .part(
                "customer",
                part_text(payload.to_string(), "application/json")?,
            )
            .part(
                "customerAccountPhoto",
                part_bytes(request.customer_account_photo.clone(), "account-photo.jpg", "image/jpeg")?,
            );
        self.request_multipart(
            "/registerCustomerAccountPhoto",
            form,
            Some(&payload),
            CUSTOMER_ACCOUNT_PHOTO_HASH_FIELDS,
            true,
        )
        .await
    }

    /// Upload a customer's identification.
    pub async fn register_customer_identification(
        &self,
        request: RegisterCustomerIdentification,
    ) -> Result<CustomerResponse, PagaError> {
        let payload = request.to_payload();
        let form = Form::new()
            .part(
                "customer",
                part_text(payload.to_string(), "application/json")?,
            )
            .part(
                "customerIdPhoto",
                part_bytes(request.customer_id_photo.clone(), "id-photo.jpg", "image/jpeg")?,
            );
        self.request_multipart(
            "/registerCustomerIdentification",
            form,
            Some(&payload),
            CUSTOMER_IDENTIFICATION_HASH_FIELDS,
            true,
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Extra Business operations                                         //
    // ----------------------------------------------------------------- //
}
