use crate::auth::{
    ACCOUNT_BALANCE_HASH_FIELDS, AIRTIME_HASH_FIELDS, BILL_PAYMENT_HASH_FIELDS, MONEY_TRANSFER_HASH_FIELDS,
    PAYMENT_REQUEST_HASH_FIELDS, PAYMENT_STATUS_HASH_FIELDS, REFUND_HASH_FIELDS, SUBSIDIARY_ACCOUNT_HASH_FIELDS,
    SUBSIDIARY_BALANCE_HASH_FIELDS, SUBSIDIARY_CHARGE_HASH_FIELDS, SUBSIDIARY_HISTORY_HASH_FIELDS,
    SUBSIDIARY_RETRIEVE_HASH_FIELDS, SUBSIDIARY_TOPUP_HASH_FIELDS, SUBSIDIARY_TRANSFER_HASH_FIELDS,
    SUBSIDIARY_UPDATE_HASH_FIELDS, TRANSACTION_HISTORY_HASH_FIELDS, compute_hash,
};
use crate::errors::{PagaAPIError, PagaAuthenticationError, PagaError, PagaRateLimitError};
use crate::types::{
    AccountBalanceResponse, CollectPaymentRequestResponse, DepositToBankResponse, MerchantPaymentResponse,
    PaymentStatusResponse, PurchaseResponse, RefundResponse, SubsidiaryAccountResponse, SubsidiaryBalanceResponse,
    SubsidiaryHistoryResponse, SubsidiaryMoneyResponse, SubsidiaryTransferResponse, TransactionHistoryResponse,
};
use reqwest::{Client, Method};
use serde_json::{json, Map, Value};

pub const DEFAULT_COLLECT_TEST_BASE_URL: &str = "https://beta-collect.paga.com";
pub const DEFAULT_COLLECT_LIVE_BASE_URL: &str = "https://collect.paga.com";
pub const DEFAULT_CHECKOUT_TEST_BASE_URL: &str = "https://beta-checkout.paga.com";
pub const DEFAULT_CHECKOUT_LIVE_BASE_URL: &str = "https://checkout.paga.com";
pub const DEFAULT_BUSINESS_TEST_BASE_URL: &str = "https://beta.mypaga.com/paga-webservices/business-rest/secured";
pub const DEFAULT_BUSINESS_LIVE_BASE_URL: &str = "https://www.mypaga.com/paga-webservices/business-rest/secured";

pub struct PagaClient {
    public_key: String,
    secret_key: String,
    hash_key: String,
    environment: String,
    base_url: String,
    checkout_base_url: String,
    business_base_url: String,
    client: Client,
}

impl PagaClient {
    pub fn new(public_key: &str, secret_key: &str, hash_key: &str, environment: &str) -> Self {
        let (base_url, checkout_base_url, business_base_url) = if environment == "live" {
            (
                DEFAULT_COLLECT_LIVE_BASE_URL,
                DEFAULT_CHECKOUT_LIVE_BASE_URL,
                DEFAULT_BUSINESS_LIVE_BASE_URL,
            )
        } else {
            (
                DEFAULT_COLLECT_TEST_BASE_URL,
                DEFAULT_CHECKOUT_TEST_BASE_URL,
                DEFAULT_BUSINESS_TEST_BASE_URL,
            )
        };
        Self {
            public_key: public_key.to_string(),
            secret_key: secret_key.to_string(),
            hash_key: hash_key.to_string(),
            environment: environment.to_string(),
            base_url: base_url.to_string(),
            checkout_base_url: checkout_base_url.to_string(),
            business_base_url: business_base_url.to_string(),
            client: Client::new(),
        }
    }

    async fn request(
        &self,
        method: &str,
        path: &str,
        payload: Option<&Value>,
        use_hash: bool,
        use_basic: bool,
        base_url: &str,
        hash_fields: &[&str],
        raise_on_error: bool,
    ) -> Result<Value, PagaError> {
        let url = format!("{}{}", base_url.trim_end_matches('/'), path);
        let mut req = self.client.request(method_to_method(method), &url);
        if use_basic {
            req = req.basic_auth(&self.public_key, Some(&self.secret_key));
        }
        if use_hash {
            if let Some(p) = payload {
                req = req.header("hash", compute_hash(Some(p), &self.hash_key, hash_fields));
                if !method.eq_ignore_ascii_case("GET") {
                    req = req.json(p);
                }
            }
        }
        let response = req.send().await.map_err(|e| PagaError::new(format!("Paga request failed: {}", e)))?;
        let status = response.status().as_u16() as i64;
        if status == 401 || status == 403 {
            return Err(PagaAuthenticationError::new(format!("Paga authentication failed with status {}", status)).into());
        }
        if status == 429 {
            return Err(PagaRateLimitError::new("Paga rate limit exceeded").into());
        }
        let body = response.json::<Value>().await.unwrap_or(Value::Null);
        if status >= 400 || (raise_on_error && body.get("responseCode").and_then(|v| v.as_i64()) != Some(0)) {
            let message = body
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Paga request failed")
                .to_string();
            let response_code = body.get("responseCode").and_then(|v| v.as_i64());
            return Err(PagaAPIError::new(message, Some(status), response_code, Some(body)).into());
        }
        Ok(body)
    }

    pub async fn create_payment(
        &self,
        reference_number: &str,
        amount: f64,
        payer_name: &str,
        payer_phone_number: &str,
        payee_name: &str,
    ) -> Result<CollectPaymentRequestResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("amount".to_string(), json!(amount)),
            ("currency".to_string(), json!("NGN")),
            (
                "payer".to_string(),
                obj(vec![
                    ("name".to_string(), json!(payer_name)),
                    ("phoneNumber".to_string(), json!(payer_phone_number)),
                ]),
            ),
            (
                "payee".to_string(),
                obj(vec![("name".to_string(), json!(payee_name))]),
            ),
            ("payerCollectionFeeShare".to_string(), json!(1.0)),
            ("payeeCollectionFeeShare".to_string(), json!(0.0)),
            ("paymentMethods".to_string(), json!(vec!["BANK_TRANSFER"])),
        ]);
        let body = self
            .request("POST", "/paymentRequest", Some(&payload), true, true, &self.base_url, &PAYMENT_REQUEST_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse payment response: {}", e)))
    }

    pub async fn get_payment_status(&self, reference_number: &str) -> Result<PaymentStatusResponse, PagaError> {
        let payload = obj(vec![("referenceNumber".to_string(), json!(reference_number))]);
        let body = self
            .request("POST", "/status", Some(&payload), true, true, &self.base_url, &PAYMENT_STATUS_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse status response: {}", e)))
    }

    pub async fn verify_charge(&self, payment_reference: &str, public_key: &str, amount: f64) -> Result<Value, PagaError> {
        let payload = obj(vec![
            ("paymentReference".to_string(), json!(payment_reference)),
            ("publicKey".to_string(), json!(public_key)),
            ("amount".to_string(), json!(amount)),
            ("currency".to_string(), json!("NGN")),
        ]);
        self.request("POST", "/checkout/transaction/verify", Some(&payload), false, true, &self.checkout_base_url, &[], true)
            .await
    }

    pub async fn refund_payment(
        &self,
        reference_number: &str,
        payment_request_reference: &str,
        refund_amount: f64,
    ) -> Result<RefundResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("paymentRequestReference".to_string(), json!(payment_request_reference)),
            ("refundAmount".to_string(), json!(refund_amount)),
            ("currency".to_string(), json!("NGN")),
        ]);
        let body = self
            .request("POST", "/refund/v2", Some(&payload), true, true, &self.base_url, &REFUND_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse refund response: {}", e)))
    }

    pub async fn send_money(
        &self,
        reference_number: &str,
        amount: &str,
        destination_bank_uuid: &str,
        destination_bank_account_number: &str,
    ) -> Result<DepositToBankResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("amount".to_string(), json!(amount)),
            ("currency".to_string(), json!("NGN")),
            ("destinationBankUUID".to_string(), json!(destination_bank_uuid)),
            ("destinationBankAccountNumber".to_string(), json!(destination_bank_account_number)),
        ]);
        let body = self
            .request("POST", "/depositToBank", Some(&payload), true, false, &self.business_base_url, &MONEY_TRANSFER_HASH_FIELDS, false)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse transfer response: {}", e)))
    }

    pub async fn buy_airtime(&self, reference_number: &str, amount: &str, destination_phone_number: &str, mobile_operator_public_id: &str) -> Result<PurchaseResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("amount".to_string(), json!(amount)),
            ("destinationPhoneNumber".to_string(), json!(destination_phone_number)),
            ("mobileOperatorPublicId".to_string(), json!(mobile_operator_public_id)),
            ("currency".to_string(), json!("NGN")),
            ("isDataBundle".to_string(), json!(false)),
        ]);
        let body = self
            .request("POST", "/airtimePurchase", Some(&payload), true, false, &self.business_base_url, &AIRTIME_HASH_FIELDS, false)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse purchase response: {}", e)))
    }

    pub async fn pay_bill(&self, reference_number: &str, amount: f64, merchant_account: &str, merchant_reference_number: &str) -> Result<MerchantPaymentResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("amount".to_string(), json!(amount)),
            ("merchantAccount".to_string(), json!(merchant_account)),
            ("merchantReferenceNumber".to_string(), json!(merchant_reference_number)),
            ("currency".to_string(), json!("NGN")),
        ]);
        let body = self
            .request("POST", "/merchantPayment", Some(&payload), true, false, &self.business_base_url, &BILL_PAYMENT_HASH_FIELDS, false)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse bill response: {}", e)))
    }

    pub async fn get_business_balance(&self, reference_number: &str) -> Result<AccountBalanceResponse, PagaError> {
        let payload = obj(vec![("referenceNumber".to_string(), json!(reference_number))]);
        let body = self
            .request("POST", "/accountBalance", Some(&payload), true, false, &self.business_base_url, &ACCOUNT_BALANCE_HASH_FIELDS, false)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse balance response: {}", e)))
    }

    pub async fn get_business_transactions(&self, reference_number: &str, start_date_utc: &str, end_date_utc: &str) -> Result<TransactionHistoryResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("startDateUTC".to_string(), json!(start_date_utc)),
            ("endDateUTC".to_string(), json!(end_date_utc)),
        ]);
        let body = self
            .request("POST", "/transactionHistory", Some(&payload), true, false, &self.business_base_url, &TRANSACTION_HISTORY_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse history response: {}", e)))
    }

    pub async fn create_subsidiary_account(&self, reference_number: &str, account_reference: &str, account_name: &str) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("accountReference".to_string(), json!(account_reference)),
            ("accountName".to_string(), json!(account_name)),
        ]);
        let body = self
            .request("POST", "/subsidiary-accounts", Some(&payload), true, true, &self.base_url, &SUBSIDIARY_ACCOUNT_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse subsidiary response: {}", e)))
    }

    pub async fn charge_subsidiary_account(&self, account_reference: &str, reference_number: &str, amount: f64) -> Result<SubsidiaryMoneyResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("amount".to_string(), json!(amount)),
            ("currency".to_string(), json!("NGN")),
        ]);
        let body = self
            .request("POST", &format!("/subsidiary-accounts/{}/charge", account_reference), Some(&payload), true, true, &self.base_url, &SUBSIDIARY_CHARGE_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse charge response: {}", e)))
    }

    pub async fn top_up_subsidiary_account(&self, account_reference: &str, reference_number: &str, amount: f64) -> Result<SubsidiaryMoneyResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("amount".to_string(), json!(amount)),
            ("currency".to_string(), json!("NGN")),
        ]);
        let body = self
            .request("POST", &format!("/subsidiary-accounts/{}/topup", account_reference), Some(&payload), true, true, &self.base_url, &SUBSIDIARY_TOPUP_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse topup response: {}", e)))
    }

    pub async fn transfer_between_subsidiary_accounts(
        &self,
        source_account_reference: &str,
        destination_account_reference: &str,
        reference_number: &str,
        amount: f64,
    ) -> Result<SubsidiaryTransferResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("sourceAccountIdentifier".to_string(), json!(source_account_reference)),
            ("destinationAccountIdentifier".to_string(), json!(destination_account_reference)),
            ("amount".to_string(), json!(amount)),
            ("currency".to_string(), json!("NGN")),
        ]);
        let body = self
            .request("POST", "/subsidiary-accounts/transfer", Some(&payload), true, true, &self.base_url, &SUBSIDIARY_TRANSFER_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse transfer response: {}", e)))
    }

    pub async fn retrieve_subsidiary_account(&self, account_reference: &str, reference_number: &str) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("accountIdentifier".to_string(), json!(account_reference)),
        ]);
        let body = self
            .request("GET", &format!("/subsidiary-accounts/{}?referenceNumber={}", account_reference, reference_number), Some(&payload), true, true, &self.base_url, &SUBSIDIARY_RETRIEVE_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse subsidiary response: {}", e)))
    }

    pub async fn update_subsidiary_account(&self, account_reference: &str, reference_number: &str, status: &str) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("accountIdentifier".to_string(), json!(account_reference)),
            ("status".to_string(), json!(status)),
        ]);
        let body = self
            .request("PUT", &format!("/subsidiary-accounts/{}", account_reference), Some(&payload), true, true, &self.base_url, &SUBSIDIARY_UPDATE_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse subsidiary response: {}", e)))
    }

    pub async fn get_subsidiary_balance(&self, account_reference: &str, reference_number: &str) -> Result<SubsidiaryBalanceResponse, PagaError> {
        let payload = obj(vec![
            ("referenceNumber".to_string(), json!(reference_number)),
            ("accountIdentifier".to_string(), json!(account_reference)),
        ]);
        let body = self
            .request("GET", &format!("/subsidiary-accounts/{}/balance?referenceNumber={}", account_reference, reference_number), Some(&payload), true, true, &self.base_url, &SUBSIDIARY_BALANCE_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse balance response: {}", e)))
    }

    pub async fn get_subsidiary_history(&self, account_reference: &str, start_date_time_utc: &str, end_date_time_utc: &str) -> Result<SubsidiaryHistoryResponse, PagaError> {
        let payload = obj(vec![
            ("accountIdentifier".to_string(), json!(account_reference)),
            ("startDateTimeUTC".to_string(), json!(start_date_time_utc)),
            ("endDateTimeUTC".to_string(), json!(end_date_time_utc)),
        ]);
        let body = self
            .request("GET", &format!("/subsidiary-accounts/{}/history?startDateTimeUTC={}&endDateTimeUTC={}", account_reference, start_date_time_utc, end_date_time_utc), Some(&payload), true, true, &self.base_url, &SUBSIDIARY_HISTORY_HASH_FIELDS, true)
            .await?;
        serde_json::from_value(body).map_err(|e| PagaError::new(format!("Failed to parse history response: {}", e)))
    }
}

fn method_to_method(method: &str) -> Method {
    match method.to_uppercase().as_str() {
        "GET" => Method::GET,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        _ => Method::POST,
    }
}

fn obj(pairs: Vec<(String, Value)>) -> Value {
    let mut map = Map::new();
    for (k, v) in pairs {
        map.insert(k, v);
    }
    Value::Object(map)
}
