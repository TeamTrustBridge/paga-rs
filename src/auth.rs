/** Authentication and hash helpers for the Paga Engine API. */

use serde_json::Value;
use sha2::{Digest, Sha512};

pub const DEFAULT_HASH_ALGORITHM: &str = "sha512";

pub const PAYMENT_REQUEST_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "currency",
    "payer.phoneNumber",
    "payer.email",
];

pub const BILL_PAYMENT_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "merchantAccount",
    "merchantReferenceNumber",
];

pub const AIRTIME_HASH_FIELDS: &[&str] = &["referenceNumber", "amount", "destinationPhoneNumber"];

pub const MONEY_TRANSFER_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "destinationBankUUID",
    "destinationBankAccountNumber",
];

pub const ACCOUNT_BALANCE_HASH_FIELDS: &[&str] = &["referenceNumber"];

pub const PAYMENT_STATUS_HASH_FIELDS: &[&str] = &["referenceNumber"];

pub const TRANSACTION_HISTORY_HASH_FIELDS: &[&str] = &["referenceNumber"];

pub const REFUND_HASH_FIELDS: &[&str] = &["referenceNumber", "refundAmount"];

pub const PAYMENT_CALLBACK_HASH_FIELDS: &[&str] = &[
    "notificationId",
    "statusCode",
    "externalReferenceNumber",
    "state",
    "outstandingBalance",
];

pub const SUBSIDIARY_ACCOUNT_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "accountReference",
    "iifiNumber",
    "autoSweep.bankPublicId",
    "autoSweep.destination",
    "autoSweep.accountNumber",
    "callbackUrl",
];

pub const SUBSIDIARY_CHARGE_HASH_FIELDS: &[&str] = &["referenceNumber", "amount", "currency", "narration"];

pub const SUBSIDIARY_TOPUP_HASH_FIELDS: &[&str] = &["referenceNumber", "amount", "currency", "narration"];

pub const SUBSIDIARY_TRANSFER_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "sourceAccountIdentifier",
    "destinationAccountIdentifier",
    "amount",
    "currency",
    "narration",
];

pub const SUBSIDIARY_RETRIEVE_HASH_FIELDS: &[&str] = &["referenceNumber", "accountIdentifier"];

pub const SUBSIDIARY_UPDATE_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "accountIdentifier",
    "iifiNumber",
    "autoSweep.destination",
    "autoSweep.bankPublicId",
    "autoSweep.accountNumber",
    "callbackUrl",
];

pub const SUBSIDIARY_BALANCE_HASH_FIELDS: &[&str] = &["referenceNumber", "accountIdentifier"];

pub const SUBSIDIARY_HISTORY_HASH_FIELDS: &[&str] = &["accountIdentifier", "startDateTimeUTC", "endDateTimeUTC"];

pub const SUBSIDIARY_FUNDING_HASH_FIELDS: &[&str] = &[
    "transactionReference",
    "transactionDate",
    "amount",
    "accountNumber",
    "timestamp",
];

pub const SUBSIDIARY_OPERATION_HASH_FIELDS: &[&str] = &[
    "externalReferenceNumber",
    "transactionDate",
    "amount",
    "accountNumber",
];

fn resolve_value<'a>(payload: Option<&'a Value>, path: &str) -> String {
    let mut node: Option<&'a Value> = payload;
    for key in path.split('.') {
        match node {
            Some(Value::Object(map)) => node = map.get(key).and_then(|v| Some(v)),
            _ => return String::new(),
        }
    }
    match node {
        Some(Value::Null) | None => String::new(),
        Some(v) => v.to_string(),
    }
}

pub fn compute_hash(payload: Option<&Value>, hash_key: &str, fields: &[&str]) -> String {
    let mut text = String::new();
    for field in fields {
        text += &resolve_value(payload, field);
    }
    text += hash_key;
    let digest = Sha512::digest(text.as_bytes());
    hex::encode(digest)
}

fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

pub fn verify_hash(payload: Option<&Value>, received_hash: &str, hash_key: &str, fields: &[&str]) -> bool {
    let computed = compute_hash(payload, hash_key, fields);
    constant_time_eq(&computed, received_hash)
}
