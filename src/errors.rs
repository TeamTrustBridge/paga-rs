/** Error types for the Paga Engine API SDK. */

use std::fmt;

use crate::codes::{
    classify, resolve_business_code, resolve_collect_status, OperationOutcome, PagaCode,
};

/// Which family of failure occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagaErrorKind {
    /// Paga rejected the credentials, or the source IP is not whitelisted.
    Authentication,
    /// Paga returned a failure response.
    Api,
    /// Paga rate-limited the request.
    RateLimit,
    /// The request never reached Paga.
    Transport,
}

/// The error returned by every fallible operation.
///
/// The specific error types below all convert into this one, so the
/// `kind` field tells you which one it was.
#[derive(Debug)]
pub struct PagaError {
    pub message: String,
    pub status_code: Option<i64>,
    pub response_code: Option<i64>,
    pub payload: Option<serde_json::Value>,
    /// Which family of failure this is.
    pub kind: PagaErrorKind,
    /// The resolved Collect status or Business error code, when the body
    /// carried one.
    pub code: Option<PagaCode>,
    /// The classified outcome of the operation.
    pub outcome: OperationOutcome,
}

impl PagaError {
    /// Build an error with no additional detail.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status_code: None,
            response_code: None,
            payload: None,
            kind: PagaErrorKind::Api,
            code: None,
            outcome: OperationOutcome::Unknown,
        }
    }

    /// Build an API error, resolving the code and outcome from the payload.
    pub fn api(
        message: impl Into<String>,
        status_code: Option<i64>,
        response_code: Option<i64>,
        payload: Option<serde_json::Value>,
    ) -> Self {
        let code = resolve_collect_status(payload.as_ref())
            .map(PagaCode::Collect)
            .or_else(|| resolve_business_code(payload.as_ref()).map(PagaCode::Business));
        let outcome = classify(payload.as_ref());
        Self {
            message: message.into(),
            status_code,
            response_code,
            payload,
            kind: PagaErrorKind::Api,
            code,
            outcome,
        }
    }

    /// Whether the outcome is undetermined.
    ///
    /// When this is true, check the transaction status using the original
    /// reference number. Do not resend with a new reference.
    pub fn is_unknown_outcome(&self) -> bool {
        self.outcome == OperationOutcome::Unknown
    }
}

impl fmt::Display for PagaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(
                f,
                "Paga API error (code={:?}, outcome={}): {}",
                code,
                self.outcome.as_str(),
                self.message
            ),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for PagaError {}

/// Paga rejected the supplied credentials or signature.
#[derive(Debug)]
pub struct PagaAuthenticationError {
    pub message: String,
}

impl PagaAuthenticationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl From<PagaAuthenticationError> for PagaError {
    fn from(e: PagaAuthenticationError) -> Self {
        PagaError {
            message: e.message,
            status_code: None,
            response_code: None,
            payload: None,
            kind: PagaErrorKind::Authentication,
            code: None,
            outcome: OperationOutcome::Unknown,
        }
    }
}

impl fmt::Display for PagaAuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Paga returned a failure response.
#[derive(Debug)]
pub struct PagaAPIError {
    pub message: String,
    pub status_code: Option<i64>,
    pub response_code: Option<i64>,
    pub payload: Option<serde_json::Value>,
}

impl PagaAPIError {
    pub fn new(
        message: impl Into<String>,
        status_code: Option<i64>,
        response_code: Option<i64>,
        payload: Option<serde_json::Value>,
    ) -> Self {
        Self {
            message: message.into(),
            status_code,
            response_code,
            payload,
        }
    }
}

impl From<PagaAPIError> for PagaError {
    fn from(e: PagaAPIError) -> Self {
        PagaError::api(e.message, e.status_code, e.response_code, e.payload)
    }
}

impl fmt::Display for PagaAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Paga rate-limited the request.
#[derive(Debug)]
pub struct PagaRateLimitError {
    pub message: String,
}

impl PagaRateLimitError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl From<PagaRateLimitError> for PagaError {
    fn from(e: PagaRateLimitError) -> Self {
        PagaError {
            message: e.message,
            status_code: Some(429),
            response_code: None,
            payload: None,
            kind: PagaErrorKind::RateLimit,
            code: None,
            outcome: OperationOutcome::Unknown,
        }
    }
}

impl fmt::Display for PagaRateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
