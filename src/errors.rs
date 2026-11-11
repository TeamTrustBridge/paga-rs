/** Error types for the Paga Engine API SDK. */

use std::fmt;

#[derive(Debug)]
pub struct PagaError {
    pub message: String,
    pub status_code: Option<i64>,
    pub response_code: Option<i64>,
    pub payload: Option<serde_json::Value>,
}

impl PagaError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status_code: None,
            response_code: None,
            payload: None,
        }
    }
}

impl fmt::Display for PagaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PagaError {}

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
        PagaError::new(e.message)
    }
}

impl fmt::Display for PagaAuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct PagaAPIError {
    pub message: String,
    pub status_code: Option<i64>,
    pub response_code: Option<i64>,
    pub payload: Option<serde_json::Value>,
}

impl PagaAPIError {
    pub fn new(message: impl Into<String>, status_code: Option<i64>, response_code: Option<i64>, payload: Option<serde_json::Value>) -> Self {
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
        PagaError {
            message: e.message,
            status_code: e.status_code,
            response_code: e.response_code,
            payload: e.payload,
        }
    }
}

impl fmt::Display for PagaAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

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
        PagaError::new(e.message)
    }
}

impl fmt::Display for PagaRateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
