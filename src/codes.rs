/** Paga status and error codes.
 *
 * Paga exposes two different code vocabularies:
 *
 * - The **Collect API** (payments, refunds, subsidiary accounts, Direct
 *   Debit) reports a `statusCode` **string**. `"0"` means success.
 * - The **Business API** (money transfer, airtime, bill payment, business
 *   account) reports a `responseCode` **integer**. `0` means success.
 *
 * They are modelled separately because the same numeric value means different
 * things in each family: `114` is `FAILED` on Collect but `INVALID_ACCOUNT`
 * on Business.
 */

use serde_json::Value;

/// `statusCode` values returned by the Collect API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectStatusCode {
    Success,
    InProcess,
    PartialPayment,
    SystemError,
    RequestExpired,
    RequestCancelled,
    Failed,
    FailedNonRefundable,
    NotFound,
}

impl CollectStatusCode {
    /// The wire value of this code.
    pub fn as_str(self) -> &'static str {
        match self {
            CollectStatusCode::Success => "0",
            CollectStatusCode::InProcess => "1",
            CollectStatusCode::PartialPayment => "2",
            CollectStatusCode::SystemError => "-1",
            CollectStatusCode::RequestExpired => "-2",
            CollectStatusCode::RequestCancelled => "-3",
            CollectStatusCode::Failed => "114",
            CollectStatusCode::FailedNonRefundable => "115",
            CollectStatusCode::NotFound => "121",
        }
    }

    /// Parse a wire value, if it is one Paga documents.
    pub fn from_str(value: &str) -> Option<Self> {
        Some(match value {
            "0" => CollectStatusCode::Success,
            "1" => CollectStatusCode::InProcess,
            "2" => CollectStatusCode::PartialPayment,
            "-1" => CollectStatusCode::SystemError,
            "-2" => CollectStatusCode::RequestExpired,
            "-3" => CollectStatusCode::RequestCancelled,
            "114" => CollectStatusCode::Failed,
            "115" => CollectStatusCode::FailedNonRefundable,
            "121" => CollectStatusCode::NotFound,
            _ => return None,
        })
    }
}

/// `responseCode` values returned by the Business API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusinessErrorCode {
    Success,
    BulkPartialSuccess,
    BulkAllFailed,
    Pending,
    UnknownError,
    UnsupportedOperation,
    InvalidAccountNumber,
    InvalidBank,
    DisabledAccount,
    InvalidPrincipal,
    DuplicateReference,
    InvalidRecipientAccount,
    InvalidOrganisationUser,
    AirtimeError,
    InvalidOrganisation,
    InvalidCustomer,
    InvalidBiller,
    InvalidUser,
    InvalidAccount,
    InvalidBankAccount,
    InvalidAmount,
    MissingParameter,
    InvalidUserIdentifier,
    InvalidProcess,
    PhotoAlreadySet,
    InvalidImageType,
    InvalidIdentificationDetails,
    InvalidPhoneNumber,
    InvalidFirstName,
    InvalidLastName,
    InvalidUsername,
    InvalidEmail,
    EmailAlreadyExists,
    LockedUserAccount,
    InvalidName,
    InvalidMerchant,
    InvalidProcessChannel,
    DestinationBankNotFound,
    InvalidDateOfBirth,
    UnknownReferenceNumber,
    ExceedsTransactionLimit,
    BelowMinimum,
    ExceedsAccountLimit,
    InsufficientBalance,
    ReferenceNumberTooLong,
}

impl BusinessErrorCode {
    /// The wire value of this code.
    pub fn as_i64(self) -> i64 {
        match self {
            BusinessErrorCode::Success => 0,
            BusinessErrorCode::BulkPartialSuccess => 1,
            BusinessErrorCode::BulkAllFailed => 2,
            BusinessErrorCode::Pending => 3,
            BusinessErrorCode::UnknownError => -1,
            BusinessErrorCode::UnsupportedOperation => -2,
            BusinessErrorCode::InvalidAccountNumber => 4,
            BusinessErrorCode::InvalidBank => 101,
            BusinessErrorCode::DisabledAccount => 103,
            BusinessErrorCode::InvalidPrincipal => 104,
            BusinessErrorCode::DuplicateReference => 105,
            BusinessErrorCode::InvalidRecipientAccount => 107,
            BusinessErrorCode::InvalidOrganisationUser => 108,
            BusinessErrorCode::AirtimeError => 109,
            BusinessErrorCode::InvalidOrganisation => 110,
            BusinessErrorCode::InvalidCustomer => 111,
            BusinessErrorCode::InvalidBiller => 112,
            BusinessErrorCode::InvalidUser => 113,
            BusinessErrorCode::InvalidAccount => 114,
            BusinessErrorCode::InvalidBankAccount => 115,
            BusinessErrorCode::InvalidAmount => 116,
            BusinessErrorCode::MissingParameter => 117,
            BusinessErrorCode::InvalidUserIdentifier => 118,
            BusinessErrorCode::InvalidProcess => 119,
            BusinessErrorCode::PhotoAlreadySet => 120,
            BusinessErrorCode::InvalidImageType => 121,
            BusinessErrorCode::InvalidIdentificationDetails => 122,
            BusinessErrorCode::InvalidPhoneNumber => 123,
            BusinessErrorCode::InvalidFirstName => 124,
            BusinessErrorCode::InvalidLastName => 125,
            BusinessErrorCode::InvalidUsername => 126,
            BusinessErrorCode::InvalidEmail => 127,
            BusinessErrorCode::EmailAlreadyExists => 128,
            BusinessErrorCode::LockedUserAccount => 129,
            BusinessErrorCode::InvalidName => 130,
            BusinessErrorCode::InvalidMerchant => 131,
            BusinessErrorCode::InvalidProcessChannel => 132,
            BusinessErrorCode::DestinationBankNotFound => 133,
            BusinessErrorCode::InvalidDateOfBirth => 134,
            BusinessErrorCode::UnknownReferenceNumber => 135,
            BusinessErrorCode::ExceedsTransactionLimit => 136,
            BusinessErrorCode::BelowMinimum => 137,
            BusinessErrorCode::ExceedsAccountLimit => 138,
            BusinessErrorCode::InsufficientBalance => 139,
            BusinessErrorCode::ReferenceNumberTooLong => 140,
        }
    }

    /// Parse a wire value, if it is one Paga documents.
    pub fn from_i64(value: i64) -> Option<Self> {
        Some(match value {
            0 => BusinessErrorCode::Success,
            1 => BusinessErrorCode::BulkPartialSuccess,
            2 => BusinessErrorCode::BulkAllFailed,
            3 => BusinessErrorCode::Pending,
            -1 => BusinessErrorCode::UnknownError,
            -2 => BusinessErrorCode::UnsupportedOperation,
            4 => BusinessErrorCode::InvalidAccountNumber,
            101 => BusinessErrorCode::InvalidBank,
            103 => BusinessErrorCode::DisabledAccount,
            104 => BusinessErrorCode::InvalidPrincipal,
            105 => BusinessErrorCode::DuplicateReference,
            107 => BusinessErrorCode::InvalidRecipientAccount,
            108 => BusinessErrorCode::InvalidOrganisationUser,
            109 => BusinessErrorCode::AirtimeError,
            110 => BusinessErrorCode::InvalidOrganisation,
            111 => BusinessErrorCode::InvalidCustomer,
            112 => BusinessErrorCode::InvalidBiller,
            113 => BusinessErrorCode::InvalidUser,
            114 => BusinessErrorCode::InvalidAccount,
            115 => BusinessErrorCode::InvalidBankAccount,
            116 => BusinessErrorCode::InvalidAmount,
            117 => BusinessErrorCode::MissingParameter,
            118 => BusinessErrorCode::InvalidUserIdentifier,
            119 => BusinessErrorCode::InvalidProcess,
            120 => BusinessErrorCode::PhotoAlreadySet,
            121 => BusinessErrorCode::InvalidImageType,
            122 => BusinessErrorCode::InvalidIdentificationDetails,
            123 => BusinessErrorCode::InvalidPhoneNumber,
            124 => BusinessErrorCode::InvalidFirstName,
            125 => BusinessErrorCode::InvalidLastName,
            126 => BusinessErrorCode::InvalidUsername,
            127 => BusinessErrorCode::InvalidEmail,
            128 => BusinessErrorCode::EmailAlreadyExists,
            129 => BusinessErrorCode::LockedUserAccount,
            130 => BusinessErrorCode::InvalidName,
            131 => BusinessErrorCode::InvalidMerchant,
            132 => BusinessErrorCode::InvalidProcessChannel,
            133 => BusinessErrorCode::DestinationBankNotFound,
            134 => BusinessErrorCode::InvalidDateOfBirth,
            135 => BusinessErrorCode::UnknownReferenceNumber,
            136 => BusinessErrorCode::ExceedsTransactionLimit,
            137 => BusinessErrorCode::BelowMinimum,
            138 => BusinessErrorCode::ExceedsAccountLimit,
            139 => BusinessErrorCode::InsufficientBalance,
            140 => BusinessErrorCode::ReferenceNumberTooLong,
            _ => return None,
        })
    }
}

/// `statusCode` values on a Direct Debit mandate callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MandateStatusCode {
    Pending,
    Verified,
    Approved,
    Rejected,
    Unknown,
    NotFound,
}

impl MandateStatusCode {
    /// The wire value of this code.
    pub fn as_str(self) -> &'static str {
        match self {
            MandateStatusCode::Pending => "003",
            MandateStatusCode::Verified => "004",
            MandateStatusCode::Approved => "0",
            MandateStatusCode::Rejected => "005",
            MandateStatusCode::Unknown => "006",
            MandateStatusCode::NotFound => "007",
        }
    }
}

/// `ProcessStatusId` values on a mandate.
///
/// This is the field to branch on. A mandate's `data.statusCode` stays
/// `"1"` / `"pending"` forever, so it tells you nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MandateProcessStatus {
    Pending,
    Verified,
    Approved,
    Rejected,
}

impl MandateProcessStatus {
    /// The wire value of this status.
    pub fn as_str(self) -> &'static str {
        match self {
            MandateProcessStatus::Pending => "PENDING",
            MandateProcessStatus::Verified => "VERIFIED",
            MandateProcessStatus::Approved => "APPROVED",
            MandateProcessStatus::Rejected => "REJECTED",
        }
    }

    /// Parse a wire value, if it is one Paga documents.
    pub fn from_str(value: &str) -> Option<Self> {
        Some(match value {
            "PENDING" => MandateProcessStatus::Pending,
            "VERIFIED" => MandateProcessStatus::Verified,
            "APPROVED" => MandateProcessStatus::Approved,
            "REJECTED" => MandateProcessStatus::Rejected,
            _ => return None,
        })
    }
}

/// `data.statusCode` values returned by `/getChargeMandateStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeStatusCode {
    Charged,
    Pending,
    Invalid,
    Unknown,
}

impl ChargeStatusCode {
    /// The wire value of this code.
    pub fn as_i64(self) -> i64 {
        match self {
            ChargeStatusCode::Charged => 0,
            ChargeStatusCode::Pending => 1,
            ChargeStatusCode::Invalid => -1,
            ChargeStatusCode::Unknown => -4,
        }
    }
}

/// The outcome of a Paga operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationOutcome {
    /// The operation completed. Money moved, or the request was created.
    Success,
    /// Accepted and still in flight. Poll the status endpoint or await the callback.
    Pending,
    /// Partially fulfilled: some of the money arrived. More may still come.
    Partial,
    /// The operation was rejected. Safe to retry with a new reference.
    Failed,
    /// The outcome is undetermined. Check status with the original reference.
    Unknown,
}

impl OperationOutcome {
    /// The lowercase name of this outcome.
    pub fn as_str(self) -> &'static str {
        match self {
            OperationOutcome::Success => "success",
            OperationOutcome::Pending => "pending",
            OperationOutcome::Partial => "partial",
            OperationOutcome::Failed => "failed",
            OperationOutcome::Unknown => "unknown",
        }
    }
}

/// Either of Paga's code families.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagaCode {
    Collect(CollectStatusCode),
    Business(BusinessErrorCode),
}

/// Business API codes that mean "unknown outcome", not "failed".
pub const UNKNOWN_OUTCOME_BUSINESS_CODES: [BusinessErrorCode; 3] = [
    BusinessErrorCode::UnknownError,
    BusinessErrorCode::DuplicateReference,
    BusinessErrorCode::UnknownReferenceNumber,
];

/// Read the Collect `statusCode` out of a response body.
pub fn resolve_collect_status(body: Option<&Value>) -> Option<CollectStatusCode> {
    let record = body?.as_object()?;
    let raw = record.get("statusCode")?;
    let text = match raw {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => return None,
    };
    CollectStatusCode::from_str(&text)
}

/// Read the Business `responseCode` out of a response body.
pub fn resolve_business_code(body: Option<&Value>) -> Option<BusinessErrorCode> {
    let record = body?.as_object()?;
    let raw = record.get("responseCode")?;
    let number = raw.as_i64().or_else(|| raw.as_str().and_then(|s| s.parse().ok()))?;
    BusinessErrorCode::from_i64(number)
}

/// Classify a response body into an [`OperationOutcome`].
///
/// The Collect API is detected by the presence of `statusCode` and the
/// Business API by `responseCode`. A body carrying neither is `Unknown`,
/// since there is nothing to conclude from.
pub fn classify(body: Option<&Value>) -> OperationOutcome {
    if let Some(status) = resolve_collect_status(body) {
        return match status {
            CollectStatusCode::Success => OperationOutcome::Success,
            CollectStatusCode::InProcess => OperationOutcome::Pending,
            CollectStatusCode::PartialPayment => OperationOutcome::Partial,
            CollectStatusCode::SystemError => OperationOutcome::Unknown,
            _ => OperationOutcome::Failed,
        };
    }

    if let Some(code) = resolve_business_code(body) {
        return match code {
            BusinessErrorCode::Success | BusinessErrorCode::BulkPartialSuccess => {
                OperationOutcome::Success
            }
            BusinessErrorCode::Pending => OperationOutcome::Pending,
            _ if UNKNOWN_OUTCOME_BUSINESS_CODES.contains(&code) => OperationOutcome::Unknown,
            _ => OperationOutcome::Failed,
        };
    }

    OperationOutcome::Unknown
}

/// Whether the outcome of a body is undetermined.
///
/// An unknown outcome means you must check the transaction status using the
/// original reference number. Retrying with a fresh reference risks moving
/// money twice.
pub fn is_unknown_outcome(body: Option<&Value>) -> bool {
    classify(body) == OperationOutcome::Unknown
}
