/** Helpers for verifying and parsing Paga payment callbacks (webhooks). */

use crate::auth::{
    PAYMENT_CALLBACK_HASH_FIELDS, SUBSIDIARY_FUNDING_HASH_FIELDS, SUBSIDIARY_OPERATION_HASH_FIELDS,
};
use serde_json::Value;

pub fn verify_checkout_notification(payload: Option<&Value>, received_hash: &str, hash_key: &str) -> bool {
    crate::auth::verify_hash(payload, received_hash, hash_key, &[
        "amount",
        "timeStamp",
        "paymentReference",
    ])
}

pub fn verify_payment_callback(payload: Option<&Value>, received_hash: &str, hash_key: &str) -> bool {
    crate::auth::verify_hash(payload, received_hash, hash_key, PAYMENT_CALLBACK_HASH_FIELDS)
}

pub fn verify_funding_notification(payload: Option<&Value>, received_hash: &str, hash_key: &str) -> bool {
    crate::auth::verify_hash(payload, received_hash, hash_key, SUBSIDIARY_FUNDING_HASH_FIELDS)
}

pub fn verify_subsidiary_operation(payload: Option<&Value>, received_hash: &str, hash_key: &str) -> bool {
    crate::auth::verify_hash(payload, received_hash, hash_key, SUBSIDIARY_OPERATION_HASH_FIELDS)
}
