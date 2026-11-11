//! The Paga Engine API client.
//!
//! Two APIs, two auth styles:
//!
//! - The **Collect API** uses HTTP Basic (`publicKey:secretKey`) plus a
//!   `hash` header, and reports success in `statusCode` (the string `"0"`).
//! - The **Business API** uses `principal` and `credentials` headers plus a
//!   `hash` header, and reports success in `responseCode` (the integer `0`).

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::auth::*;
use crate::codes::{classify, OperationOutcome};
use crate::errors::{PagaAPIError, PagaAuthenticationError, PagaError, PagaErrorKind, PagaRateLimitError};
use crate::requests::*;
use crate::types::*;

pub const DEFAULT_COLLECT_TEST_BASE_URL: &str = "https://beta-collect.paga.com";
pub const DEFAULT_COLLECT_LIVE_BASE_URL: &str = "https://collect.paga.com";
pub const DEFAULT_CHECKOUT_TEST_BASE_URL: &str = "https://beta-checkout.paga.com";
pub const DEFAULT_CHECKOUT_LIVE_BASE_URL: &str = "https://checkout.paga.com";
pub const DEFAULT_BUSINESS_TEST_BASE_URL: &str =
    "https://beta.mypaga.com/paga-webservices/business-rest/secured";
pub const DEFAULT_BUSINESS_LIVE_BASE_URL: &str =
    "https://www.mypaga.com/paga-webservices/business-rest/secured";

/// The environment a client talks to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Test,
    Live,
}

impl Environment {
    /// Parse an environment name, defaulting to `test`.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "test" => Some(Environment::Test),
            "live" => Some(Environment::Live),
            _ => None,
        }
    }
}

/// Options for the client.
#[derive(Debug, Clone)]
pub struct PagaClientOptions {
    pub environment: Environment,
    pub base_url: Option<String>,
    pub checkout_base_url: Option<String>,
    pub business_base_url: Option<String>,
    /// Request timeout, in seconds.
    pub timeout_seconds: u64,
    /// How many times to replay a rate-limited (HTTP 429) request.
    ///
    /// Defaults to `0`. Server errors are never retried: a 5xx or a timeout
    /// leaves the outcome unknown, and Paga's guidance is to check the
    /// transaction status with the original reference rather than send again.
    pub max_retries: u32,
    /// Base delay, in milliseconds, for the exponential retry backoff.
    pub retry_backoff_ms: u64,
}

impl Default for PagaClientOptions {
    fn default() -> Self {
        Self {
            environment: Environment::Test,
            base_url: None,
            checkout_base_url: None,
            business_base_url: None,
            timeout_seconds: 30,
            max_retries: 0,
            retry_backoff_ms: 500,
        }
    }
}

/// How a request should be authenticated and signed.
#[derive(Debug, Clone, Copy)]
pub enum AuthStyle {
    /// The `principal` / `credentials` headers plus a `hash` header.
    Secured,
    /// HTTP Basic plus a `hash` header.
    Basic,
}

/// The options that shape a single request.
#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub auth: AuthStyle,
    pub base_url: Option<String>,
    pub hash_fields: Vec<String>,
    /// Send the `hash` header. Some endpoints use Basic auth with no hash.
    pub send_hash: bool,
    /// Raise when the body reports a failure.
    pub raise_on_error: bool,
    /// Query string parameters.
    pub params: Vec<(String, String)>,
    /// The payload the hash is computed over, when it is not the request body.
    pub hash_payload: Option<Value>,
}

impl RequestOptions {
    /// A secured (Business API) request over the given hash fields.
    pub fn secured(hash_fields: &[&str]) -> Self {
        Self {
            auth: AuthStyle::Secured,
            base_url: None,
            hash_fields: hash_fields.iter().map(|field| field.to_string()).collect(),
            send_hash: true,
            raise_on_error: true,
            params: Vec::new(),
            hash_payload: None,
        }
    }

    /// A Basic-auth (Collect API) request over the given hash fields.
    pub fn basic(hash_fields: &[&str]) -> Self {
        Self {
            auth: AuthStyle::Basic,
            base_url: None,
            hash_fields: hash_fields.iter().map(|field| field.to_string()).collect(),
            send_hash: true,
            raise_on_error: true,
            params: Vec::new(),
            hash_payload: None,
        }
    }

    /// Send HTTP Basic auth with no `hash` header.
    ///
    /// The Checkout verify and hosted set-up session endpoints work this
    /// way.
    pub fn basic_without_hash() -> Self {
        Self {
            auth: AuthStyle::Basic,
            base_url: None,
            hash_fields: Vec::new(),
            send_hash: false,
            raise_on_error: true,
            params: Vec::new(),
            hash_payload: None,
        }
    }

    /// Return the response body instead of raising on a failure.
    pub fn lenient(mut self) -> Self {
        self.raise_on_error = false;
        self
    }

    /// Override the target base URL.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Add a query string parameter.
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.push((key.into(), value.into()));
        self
    }

    /// Sign a payload other than the request body.
    pub fn hash_payload(mut self, payload: Value) -> Self {
        self.hash_payload = Some(payload);
        self
    }
}

/// An async client for the Paga Engine API.
#[derive(Debug, Clone)]
pub struct PagaClient {
    public_key: String,
    secret_key: String,
    hash_key: String,
    environment: Environment,
    base_url: String,
    checkout_base_url: String,
    business_base_url: String,
    max_retries: u32,
    retry_backoff_ms: u64,
    client: Client,
}

impl PagaClient {
    /// Build a client for the given environment.
    pub fn new(
        public_key: &str,
        secret_key: &str,
        hash_key: &str,
        environment: &str,
    ) -> Result<Self, PagaError> {
        let environment = Environment::parse(environment).ok_or_else(|| {
            PagaError::new("environment must be either \"test\" or \"live\"".to_string())
        })?;
        Ok(Self::with_options(
            public_key,
            secret_key,
            hash_key,
            PagaClientOptions {
                environment,
                ..Default::default()
            },
        ))
    }

    /// Build a client with explicit options.
    pub fn with_options(
        public_key: &str,
        secret_key: &str,
        hash_key: &str,
        options: PagaClientOptions,
    ) -> Self {
        let (collect, checkout, business) = match options.environment {
            Environment::Live => (
                DEFAULT_COLLECT_LIVE_BASE_URL,
                DEFAULT_CHECKOUT_LIVE_BASE_URL,
                DEFAULT_BUSINESS_LIVE_BASE_URL,
            ),
            Environment::Test => (
                DEFAULT_COLLECT_TEST_BASE_URL,
                DEFAULT_CHECKOUT_TEST_BASE_URL,
                DEFAULT_BUSINESS_TEST_BASE_URL,
            ),
        };

        Self {
            public_key: public_key.to_string(),
            secret_key: secret_key.to_string(),
            hash_key: hash_key.to_string(),
            environment: options.environment,
            base_url: options.base_url.unwrap_or_else(|| collect.to_string()),
            checkout_base_url: options
                .checkout_base_url
                .unwrap_or_else(|| checkout.to_string()),
            business_base_url: options
                .business_base_url
                .unwrap_or_else(|| business.to_string()),
            max_retries: options.max_retries,
            retry_backoff_ms: options.retry_backoff_ms,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(options.timeout_seconds))
                .build()
                .unwrap_or_default(),
        }
    }

    /// The environment this client targets.
    pub fn environment(&self) -> Environment {
        self.environment
    }

    /// The base URL of the collect environment.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// The base URL of the checkout environment.
    pub fn checkout_base_url(&self) -> &str {
        &self.checkout_base_url
    }

    /// The base URL of the business environment.
    pub fn business_base_url(&self) -> &str {
        &self.business_base_url
    }

    fn basic_auth_value(&self) -> String {
        use base64::Engine;
        let raw = format!("{}:{}", self.public_key, self.secret_key);
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(raw)
        )
    }

    fn build_headers(
        &self,
        signed: Option<&Value>,
        options: &RequestOptions,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();
        match options.auth {
            AuthStyle::Basic => {
                if let Ok(value) = HeaderValue::from_str(&self.basic_auth_value()) {
                    headers.insert(AUTHORIZATION, value);
                }
                if options.send_hash {
                    if let Some(payload) = signed {
                        let hash = compute_hash(
                            Some(payload),
                            &self.hash_key,
                            &options.hash_fields.iter().map(|f| f.as_str()).collect::<Vec<_>>(),
                        );
                        if let Ok(value) = HeaderValue::from_str(&hash) {
                            headers.insert(
                                HeaderName::from_static("hash"),
                                value,
                            );
                        }
                    }
                }
            }
            AuthStyle::Secured => {
                if let Ok(value) = HeaderValue::from_str(&self.public_key) {
                    headers.insert(HeaderName::from_static("principal"), value);
                }
                if let Ok(value) = HeaderValue::from_str(&self.secret_key) {
                    headers.insert(HeaderName::from_static("credentials"), value);
                }
                if options.send_hash {
                    let payload = signed.unwrap_or(&Value::Null);
                    let hash = compute_hash(
                        Some(payload),
                        &self.hash_key,
                        &options.hash_fields.iter().map(|f| f.as_str()).collect::<Vec<_>>(),
                    );
                    if let Ok(value) = HeaderValue::from_str(&hash) {
                        headers.insert(HeaderName::from_static("hash"), value);
                    }
                }
            }
        }
        headers
    }

    /// Send a JSON request to Paga.
    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        payload: Option<&Value>,
        options: RequestOptions,
    ) -> Result<T, PagaError> {
        let base = options
            .base_url
            .clone()
            .unwrap_or_else(|| self.base_url.clone());
        let url = format!("{}{}", base.trim_end_matches('/'), path);

        let signed = options.hash_payload.as_ref().or(payload);
        let headers = self.build_headers(signed, &options);
        let mut request = self.client.request(method, &url).headers(headers);
        if !options.params.is_empty() {
            request = request.query(&options.params);
        }
        if let Some(body) = payload {
            request = request.json(body);
        }

        let response = self.send_with_retries(request).await?;
        self.finish::<T>(response, options.raise_on_error).await
    }

    /// Send a `multipart/form-data` request to Paga.
    async fn request_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        form: Form,
        hash_payload: Option<&Value>,
        hash_fields: &[&str],
        raise_on_error: bool,
    ) -> Result<T, PagaError> {
        let url = format!("{}{}", self.business_base_url.trim_end_matches('/'), path);

        let mut options = RequestOptions::secured(hash_fields);
        options.hash_payload = hash_payload.cloned();
        let headers = self.build_headers(hash_payload, &options);

        let request = self
            .client
            .post(&url)
            .headers(headers)
            .multipart(form);

        let response = self.send_with_retries(request).await?;
        self.finish::<T>(response, raise_on_error).await
    }

    /// Send a request, replaying rate-limited calls when enabled.
    ///
    /// A multipart body cannot be replayed, so such a request is sent once.
    async fn send_with_retries(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, PagaError> {
        // Keep a spare copy for retries. `try_clone` fails for multipart
        // bodies, which then simply do not retry.
        let spare = request.try_clone();
        let mut current = request;
        let mut attempt = 0u32;

        loop {
            let response = current.send().await.map_err(|e| {
                let mut error = PagaError::new(format!("Paga request failed: {}", e));
                error.kind = PagaErrorKind::Transport;
                error
            })?;

            if response.status() != StatusCode::TOO_MANY_REQUESTS || attempt >= self.max_retries {
                return Ok(response);
            }

            let Some(next) = spare.as_ref().and_then(|builder| builder.try_clone()) else {
                return Ok(response);
            };

            let delay = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<u64>().ok())
                .map(|seconds| seconds * 1000)
                .unwrap_or_else(|| self.retry_backoff_ms * 2u64.pow(attempt));

            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            current = next;
            attempt += 1;
        }
    }

    /// Turn a response into a parsed body, raising the right error type.
    async fn finish<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
        raise_on_error: bool,
    ) -> Result<T, PagaError> {
        let status = response.status();
        let body: Value = response.json().await.unwrap_or(Value::Null);

        if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
            return Err(PagaAuthenticationError::new(format!(
                "Paga authentication failed with status {}",
                status.as_u16()
            ))
            .into());
        }
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(PagaRateLimitError::new("Paga rate limit exceeded").into());
        }
        if status.is_client_error() || status.is_server_error() {
            return Err(PagaAPIError::new(
                read_message(&body).unwrap_or_else(|| "Paga request failed".to_string()),
                Some(status.as_u16() as i64),
                read_number(body.get("responseCode")),
                Some(body),
            )
            .into());
        }

        if raise_on_error && classify(Some(&body)) != OperationOutcome::Success {
            return Err(PagaAPIError::new(
                read_message(&body).unwrap_or_else(|| "Paga request failed".to_string()),
                Some(status.as_u16() as i64),
                read_number(body.get("responseCode")),
                Some(body),
            )
            .into());
        }

        serde_json::from_value(body)
            .map_err(|e| PagaError::new(format!("Failed to parse Paga response: {}", e)))
    }

    // ----------------------------------------------------------------- //
    // Checkout                                                           //
    // ----------------------------------------------------------------- //

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

    /// Send money to a bank account or Paga wallet.
    pub async fn send_money(&self, request: SendMoney) -> Result<DepositToBankResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/depositToBank",
            Some(&payload),
            RequestOptions::secured(MONEY_TRANSFER_HASH_FIELDS)
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

    /// Check the business account balance.
    pub async fn get_business_balance(
        &self,
        reference_number: &str,
    ) -> Result<AccountBalanceResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/accountBalance",
            Some(&payload),
            RequestOptions::secured(ACCOUNT_BALANCE_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Get the business transaction history.
    pub async fn get_business_transactions(
        &self,
        request: GetBusinessTransactions,
    ) -> Result<TransactionHistoryResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/transactionHistory",
            Some(&payload),
            RequestOptions::secured(TRANSACTION_HISTORY_HASH_FIELDS)
                .base_url(self.business_base_url.clone()),
        )
        .await
    }

    /// Get the five most recent transactions on the business account.
    pub async fn get_recent_transactions(
        &self,
        request: GetRecentTransactions,
    ) -> Result<RecentTransactionsResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/recentTransactionHistory",
            Some(&payload),
            RequestOptions::secured(RECENT_TRANSACTION_HISTORY_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Check an operation's status.
    pub async fn get_operation_status(
        &self,
        request: GetTransactionStatus,
    ) -> Result<OperationStatusResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/getOperationStatus",
            Some(&payload),
            RequestOptions::secured(GET_OPERATION_STATUS_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Onboard a merchant.
    pub async fn onboard_merchant(
        &self,
        request: OnboardMerchant,
    ) -> Result<OnboardMerchantResponse, PagaError> {
        let payload = request.to_payload();
        let hash_payload = request.to_hash_payload();
        self.request(
            Method::POST,
            "/onboardMerchant",
            Some(&payload),
            RequestOptions::secured(ONBOARD_MERCHANT_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .hash_payload(hash_payload)
                .lenient(),
        )
        .await
    }

    /// Dispense cash against a customer's withdrawal code.
    pub async fn dispense_cash(
        &self,
        request: DispenseCash,
    ) -> Result<DispenseCashResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/dispenseCash",
            Some(&payload),
            RequestOptions::secured(DISPENSE_CASH_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Subsidiary accounts                                               //
    // ----------------------------------------------------------------- //

    /// Create a subsidiary account.
    pub async fn create_subsidiary_account(
        &self,
        request: CreateSubsidiaryAccount,
    ) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/subsidiary-accounts",
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_ACCOUNT_HASH_FIELDS),
        )
        .await
    }

    /// Charge a ledger-backed subsidiary account.
    pub async fn charge_subsidiary_account(
        &self,
        request: ChargeSubsidiaryAccount,
    ) -> Result<SubsidiaryMoneyResponse, PagaError> {
        let payload = request.to_payload();
        let path = format!(
            "/subsidiary-accounts/{}/charge",
            request.account_reference
        );
        self.request(
            Method::POST,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_CHARGE_HASH_FIELDS),
        )
        .await
    }

    /// Top up a ledger-backed subsidiary account.
    pub async fn top_up_subsidiary_account(
        &self,
        request: TopUpSubsidiaryAccount,
    ) -> Result<SubsidiaryMoneyResponse, PagaError> {
        let payload = request.to_payload();
        let path = format!("/subsidiary-accounts/{}/topup", request.account_reference);
        self.request(
            Method::POST,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_TOPUP_HASH_FIELDS),
        )
        .await
    }

    /// Transfer money between two subsidiary accounts.
    pub async fn transfer_between_subsidiary_accounts(
        &self,
        request: TransferBetweenSubsidiaryAccounts,
    ) -> Result<SubsidiaryTransferResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/subsidiary-accounts/transfer",
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_TRANSFER_HASH_FIELDS),
        )
        .await
    }

    /// Retrieve a subsidiary account.
    pub async fn retrieve_subsidiary_account(
        &self,
        account_reference: &str,
        reference_number: &str,
    ) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "accountIdentifier": account_reference,
        });
        let path = format!("/subsidiary-accounts/{}", account_reference);
        self.request(
            Method::GET,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_RETRIEVE_HASH_FIELDS)
                .base_url(self.base_url.clone())
                .param("referenceNumber", reference_number),
        )
        .await
    }

    /// Update a subsidiary account.
    pub async fn update_subsidiary_account(
        &self,
        request: UpdateSubsidiaryAccount,
    ) -> Result<SubsidiaryAccountResponse, PagaError> {
        let payload = request.to_payload();
        let path = format!("/subsidiary-accounts/{}", request.account_reference);
        self.request(
            Method::PUT,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_UPDATE_HASH_FIELDS),
        )
        .await
    }

    /// Get a subsidiary account balance.
    pub async fn get_subsidiary_balance(
        &self,
        account_reference: &str,
        reference_number: &str,
    ) -> Result<SubsidiaryBalanceResponse, PagaError> {
        let payload = serde_json::json!({
            "referenceNumber": reference_number,
            "accountIdentifier": account_reference,
        });
        let path = format!("/subsidiary-accounts/{}/balance", account_reference);
        self.request(
            Method::GET,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_BALANCE_HASH_FIELDS)
                .base_url(self.base_url.clone())
                .param("referenceNumber", reference_number),
        )
        .await
    }

    /// Get a subsidiary account transaction history.
    pub async fn get_subsidiary_history(
        &self,
        request: GetSubsidiaryHistory,
    ) -> Result<SubsidiaryHistoryResponse, PagaError> {
        let payload = request.to_payload();
        let path = format!(
            "/subsidiary-accounts/{}/history",
            request.account_reference
        );
        self.request(
            Method::GET,
            &path,
            Some(&payload),
            RequestOptions::basic(SUBSIDIARY_HISTORY_HASH_FIELDS)
                .base_url(self.base_url.clone())
                .param("referenceNumber", request.reference_number.clone())
                .param("startDateTimeUTC", request.start_date_time_utc.clone())
                .param("endDateTimeUTC", request.end_date_time_utc.clone())
                .param("pageSize", request.page_size.unwrap_or(20).to_string())
                .param("pageNumber", request.page_number.unwrap_or(0).to_string()),
        )
        .await
    }

    // ----------------------------------------------------------------- //
    // Directory lookups                                                 //
    // ----------------------------------------------------------------- //

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

    /// List every institution you can pay.
    pub async fn get_banks(&self, reference_number: &str) -> Result<BanksResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/getBanks",
            Some(&payload),
            RequestOptions::secured(BANKS_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Validate a destination bank account.
    pub async fn validate_deposit_to_bank(
        &self,
        request: ValidateDepositToBank,
    ) -> Result<ValidateDepositToBankResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/validateDepositToBank",
            Some(&payload),
            RequestOptions::secured(VALIDATE_DEPOSIT_TO_BANK_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

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

    /// Send money to a Paga wallet.
    pub async fn transfer_to_paga_wallet(
        &self,
        request: TransferToPagaWallet,
    ) -> Result<MoneyTransferResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/moneyTransfer",
            Some(&payload),
            RequestOptions::secured(PAGA_WALLET_TRANSFER_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Validate a Paga wallet transfer.
    pub async fn validate_money_transfer(
        &self,
        request: ValidateMoneyTransfer,
    ) -> Result<ValidateMoneyTransferResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/validateMoneyTransfer",
            Some(&payload),
            RequestOptions::secured(VALIDATE_MONEY_TRANSFER_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Send up to 300 money transfers in one call.
    pub async fn send_bulk_money_transfer(
        &self,
        request: SendBulkMoneyTransfer,
    ) -> Result<BulkMoneyTransferResponse, PagaError> {
        if request.items.is_empty() {
            return Err(PagaError::new("items must not be empty"));
        }
        if request.items.len() > 300 {
            return Err(PagaError::new(
                "a bulk money transfer may contain at most 300 items",
            ));
        }
        let payload = request.to_payload();
        let hash_payload = request.to_hash_payload();
        self.request(
            Method::POST,
            "/moneyTransferBulk",
            Some(&payload),
            RequestOptions::secured(MONEY_TRANSFER_BULK_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .hash_payload(hash_payload)
                .lenient(),
        )
        .await
    }
}

/// Read a message off a response body.
fn read_message(body: &Value) -> Option<String> {
    body.get("message")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

/// Read a numeric code off a response body.
fn read_number(body: Option<&Value>) -> Option<i64> {
    body.and_then(|value| value.as_i64())
}

/// Build a multipart part, mapping reqwest's error into a Paga error.
fn part_text(text: String, mime: &str) -> Result<Part, PagaError> {
    Part::text(text)
        .mime_str(mime)
        .map_err(|e| PagaError::new(format!("Failed to build a multipart part: {}", e)))
}

/// Build a multipart file part, mapping reqwest's error into a Paga error.
fn part_bytes(bytes: Vec<u8>, file_name: &str, mime: &str) -> Result<Part, PagaError> {
    Part::bytes(bytes)
        .file_name(file_name.to_string())
        .mime_str(mime)
        .map_err(|e| PagaError::new(format!("Failed to build a multipart part: {}", e)))
}
