/** Helpers for verifying and parsing Paga payment callbacks (webhooks). */

use serde_json::Value;

use crate::auth::{
    parse_hash_parameters, verify_hash, verify_hash_header, CHARGE_CALLBACK_HASH_FIELDS,
    CHECKOUT_NOTIFICATION_HASH_FIELDS, MANDATE_CALLBACK_HASH_FIELDS,
    MONEY_TRANSFER_NOTIFICATION_HASH_FIELDS, PAYMENT_CALLBACK_HASH_FIELDS,
    PERSISTENT_PAYMENT_ACCOUNT_HASH_FIELDS, SUBSIDIARY_FUNDING_HASH_FIELDS,
    SUBSIDIARY_OPERATION_HASH_FIELDS,
};
use crate::types::{MoneyTransferNotification, PagaPaymentCallback, UsdPaymentWebhook};

/// Verify a checkout payment notification.
///
/// Hash: `amount + timeStamp + paymentReference + hashKey`.
pub fn verify_checkout_notification(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
) -> bool {
    verify_hash(payload, received_hash, hash_key, CHECKOUT_NOTIFICATION_HASH_FIELDS)
}

/// Verify a dynamic virtual account payment callback.
///
/// Hash: `notificationId + statusCode + externalReferenceNumber + state +
/// outstandingBalance + hashKey`.
pub fn verify_payment_callback(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
) -> bool {
    verify_hash(payload, received_hash, hash_key, PAYMENT_CALLBACK_HASH_FIELDS)
}

/// Verify a subsidiary account funding notification.
///
/// Hash: `transactionReference + transactionDate + amount + accountNumber +
/// timestamp + hashKey`. The hash travels in the `x-paga-hash-v3` header.
pub fn verify_funding_notification(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
) -> bool {
    verify_hash(payload, received_hash, hash_key, SUBSIDIARY_FUNDING_HASH_FIELDS)
}

/// Verify a subsidiary charge, top-up or transfer callback.
///
/// Hash: `externalReferenceNumber + transactionDate + amount + accountNumber +
/// hashKey`.
pub fn verify_subsidiary_operation(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
) -> bool {
    verify_hash(payload, received_hash, hash_key, SUBSIDIARY_OPERATION_HASH_FIELDS)
}

/// Verify a money transfer notification.
///
/// The digest travels in the `x-paga-hash` **header**, not the body. The
/// signed fields are `referenceNumber + transactionDate + transactionId +
/// amount + sessionId`, unless `x-paga-hash-parameters` says otherwise.
///
/// Answer HTTP 200 with `{"status": "SUCCESS"}` once you have accepted the
/// notification; Paga retries until it gets that.
pub fn verify_money_transfer_notification(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
    hash_parameters: Option<&str>,
) -> bool {
    let parameters = hash_parameters.map(parse_hash_parameters);
    verify_hash_header(
        payload,
        received_hash,
        hash_key,
        MONEY_TRANSFER_NOTIFICATION_HASH_FIELDS,
        parameters.as_deref(),
    )
}

/// Verify a persistent payment account callback.
///
/// Paga signs two digests over the same callback. The body's `hash` field
/// covers `statusCode + accountNumber + amount + clearingFeeAmount +
/// transferFeeAmount`; the `x-paga-hash` header covers
/// `transactionReference + accountNumber + amount`.
pub fn verify_persistent_payment_account_callback(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
) -> bool {
    verify_hash(
        payload,
        received_hash,
        hash_key,
        PERSISTENT_PAYMENT_ACCOUNT_HASH_FIELDS,
    )
}

/// Verify a Direct Debit mandate callback.
///
/// Hash: `statusCode + timeStamp + accountNumber + bankName +
/// referenceNumber + mandateStatus`. The `timeStamp` carries nanosecond
/// precision and must be hashed exactly as received.
pub fn verify_mandate_callback(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
) -> bool {
    verify_hash(payload, received_hash, hash_key, MANDATE_CALLBACK_HASH_FIELDS)
}

/// Verify a Direct Debit charge callback.
///
/// Hash: `statusCode + referenceNumber + accountReference + amount`. The
/// amount is a string with two decimals and no commas, such as `"600.00"`.
pub fn verify_charge_callback(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
) -> bool {
    verify_hash(payload, received_hash, hash_key, CHARGE_CALLBACK_HASH_FIELDS)
}

/// Parse a payment callback payload.
pub fn parse_payment_callback(payload: Option<&Value>) -> PagaPaymentCallback {
    serde_json::from_value(payload.cloned().unwrap_or(Value::Null)).unwrap_or_default()
}

/// Parse a money transfer notification payload.
pub fn parse_money_transfer_notification(payload: Option<&Value>) -> MoneyTransferNotification {
    serde_json::from_value(payload.cloned().unwrap_or(Value::Null)).unwrap_or_default()
}

/// Parse a USD (ACH, Wire, Swift) payment webhook.
///
/// This webhook carries **no signature**, so there is nothing to verify.
/// De-duplicate on `referenceNumber` plus `paymentId`.
pub fn parse_usd_payment_webhook(payload: Option<&Value>) -> UsdPaymentWebhook {
    serde_json::from_value(payload.cloned().unwrap_or(Value::Null)).unwrap_or_default()
}
