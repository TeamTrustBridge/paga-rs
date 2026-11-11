//! The Paga Engine API client.
//!
//! Two APIs, two auth styles:
//!
//! - The **Collect API** uses HTTP Basic (`publicKey:secretKey`) plus a
//!   `hash` header, and reports success in `statusCode` (the string `"0"`).
//! - The **Business API** uses `principal` and `credentials` headers plus a
//!   `hash` header, and reports success in `responseCode` (the integer `0`).
//!
//! This module holds the client itself and the HTTP plumbing. The operations
//! live in [`crate::operations`], one module per section of Paga's
//! documentation.

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};
use reqwest::multipart::{Form, Part};
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::auth::*;
use crate::codes::{classify, OperationOutcome};
use crate::errors::{PagaAPIError, PagaAuthenticationError, PagaError, PagaErrorKind, PagaRateLimitError};

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
    pub(crate) public_key: String,
    pub(crate) secret_key: String,
    pub(crate) hash_key: String,
    pub(crate) environment: Environment,
    pub(crate) base_url: String,
    pub(crate) checkout_base_url: String,
    pub(crate) business_base_url: String,
    pub(crate) max_retries: u32,
    pub(crate) retry_backoff_ms: u64,
    pub(crate) client: Client,
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

    pub(crate) fn basic_auth_value(&self) -> String {
        use base64::Engine;
        let raw = format!("{}:{}", self.public_key, self.secret_key);
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(raw)
        )
    }

    pub(crate) fn build_headers(
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
    pub(crate) async fn request<T: DeserializeOwned>(
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
    pub(crate) async fn request_multipart<T: DeserializeOwned>(
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
    pub(crate) async fn send_with_retries(
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
    pub(crate) async fn finish<T: DeserializeOwned>(
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
}

/// Read a message off a response body.
pub(crate) fn read_message(body: &Value) -> Option<String> {
    body.get("message")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

/// Read a numeric code off a response body.
pub(crate) fn read_number(body: Option<&Value>) -> Option<i64> {
    body.and_then(|value| value.as_i64())
}

/// Build a multipart part, mapping reqwest's error into a Paga error.
pub(crate) fn part_text(text: String, mime: &str) -> Result<Part, PagaError> {
    Part::text(text)
        .mime_str(mime)
        .map_err(|e| PagaError::new(format!("Failed to build a multipart part: {}", e)))
}

/// Build a multipart file part, mapping reqwest's error into a Paga error.
pub(crate) fn part_bytes(bytes: Vec<u8>, file_name: &str, mime: &str) -> Result<Part, PagaError> {
    Part::bytes(bytes)
        .file_name(file_name.to_string())
        .mime_str(mime)
        .map_err(|e| PagaError::new(format!("Failed to build a multipart part: {}", e)))
}
