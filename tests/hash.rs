//! Hash signing tests.
//!
//! The digests in `fixtures/hash_vectors.json` are generated outside the SDK
//! (`printf '%s' "$message" | sha512sum`), so these tests pin the SDK's
//! byte-level behaviour rather than merely confirming it agrees with itself.

mod support;

use std::fs;

use paga_rs::auth::{
    compute_hash, verify_hash, AIRTIME_HASH_FIELDS, BILL_PAYMENT_HASH_FIELDS,
    CHARGE_CALLBACK_HASH_FIELDS, CHARGE_MANDATE_HASH_FIELDS, DISABLE_MANDATE_HASH_FIELDS,
    MANDATE_CALLBACK_HASH_FIELDS, MONEY_TRANSFER_HASH_FIELDS, PAYMENT_CALLBACK_HASH_FIELDS,
    PAYMENT_REQUEST_HASH_FIELDS, REFUND_HASH_FIELDS, SUBSIDIARY_OPERATION_HASH_FIELDS,
};
use paga_rs::webhooks::{verify_charge_callback, verify_mandate_callback};
use serde_json::Value;

const HASH_KEY: &str = "hashkey123";

fn vectors() -> Value {
    let candidates = [
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/hash_vectors.json"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/../fixtures/hash_vectors.json"),
    ];
    for path in candidates {
        if let Ok(text) = fs::read_to_string(path) {
            if let Ok(value) = serde_json::from_str(&text) {
                return value;
            }
        }
    }
    panic!("shared hash vectors not found");
}

#[test]
fn signs_every_shared_vector_to_its_recorded_digest() {
    let fixture = vectors();
    let key = fixture["hashKey"].as_str().unwrap();
    for vector in fixture["vectors"].as_array().unwrap() {
        let name = vector["name"].as_str().unwrap();
        let fields: Vec<&str> = vector["fields"]
            .as_array()
            .unwrap()
            .iter()
            .map(|field| field.as_str().unwrap())
            .collect();
        let digest = compute_hash(Some(&vector["payload"]), key, &fields);
        assert_eq!(digest, vector["digest"].as_str().unwrap(), "{}", name);
    }
}

#[test]
fn produces_a_lowercase_hex_sha512_digest() {
    let fixture = vectors();
    let key = fixture["hashKey"].as_str().unwrap();
    let vector = &fixture["vectors"][0];
    let fields: Vec<&str> = vector["fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|field| field.as_str().unwrap())
        .collect();
    let digest = compute_hash(Some(&vector["payload"]), key, &fields);
    assert_eq!(digest.len(), 128);
    assert_eq!(digest, digest.to_lowercase());
    assert!(digest.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn signs_a_missing_field_as_an_empty_string() {
    let payload = serde_json::json!({ "referenceNumber": "REF-1", "amount": 100 });
    let digest = compute_hash(
        Some(&payload),
        HASH_KEY,
        &["referenceNumber", "amount", "currency"],
    );
    let expected = sha512_hex(&format!("REF-1{}{}{}", 100, "", HASH_KEY));
    assert_eq!(digest, expected);
}

#[test]
fn signs_string_values_verbatim_without_quoting_them() {
    // A string amount is signed exactly as sent, commas and all.
    let payload = serde_json::json!({ "amount": "1,000.00" });
    let digest = compute_hash(Some(&payload), HASH_KEY, &["amount"]);
    assert_eq!(digest, sha512_hex(&format!("1,000.00{}", HASH_KEY)));
}

#[test]
fn signs_a_string_amount_verbatim() {
    let payload = serde_json::json!({ "amount": "5000.00" });
    let digest = compute_hash(Some(&payload), HASH_KEY, &["amount"]);
    assert_eq!(digest, sha512_hex(&format!("5000.00{}", HASH_KEY)));
}

#[test]
fn formats_a_balance_field_to_two_decimals() {
    let payload = serde_json::json!({ "outstandingBalance": 1000.0 });
    let digest = compute_hash(Some(&payload), HASH_KEY, &["outstandingBalance"]);
    assert_eq!(digest, sha512_hex(&format!("1000.00{}", HASH_KEY)));
}

#[test]
fn resolves_nested_field_paths() {
    let payload = serde_json::json!({
        "referenceNumber": "REF-1",
        "payer": { "phoneNumber": "08031234567" }
    });
    let digest = compute_hash(
        Some(&payload),
        HASH_KEY,
        &["referenceNumber", "payer.phoneNumber"],
    );
    assert_eq!(
        digest,
        sha512_hex(&format!("REF-108031234567{}", HASH_KEY))
    );
}

#[test]
fn changes_the_digest_when_the_field_order_changes() {
    let payload = serde_json::json!({ "referenceNumber": "REF-1", "amount": 100 });
    let forward = compute_hash(Some(&payload), HASH_KEY, &["referenceNumber", "amount"]);
    let backward = compute_hash(Some(&payload), HASH_KEY, &["amount", "referenceNumber"]);
    assert_ne!(forward, backward);
}

#[test]
fn keeps_the_documented_hash_field_orders() {
    assert_eq!(&PAYMENT_REQUEST_HASH_FIELDS[..2], &["referenceNumber", "amount"]);
    assert_eq!(
        &BILL_PAYMENT_HASH_FIELDS[..3],
        &["referenceNumber", "amount", "merchantAccount"]
    );
    assert_eq!(
        AIRTIME_HASH_FIELDS,
        &["referenceNumber", "amount", "destinationPhoneNumber"]
    );
    assert_eq!(
        MONEY_TRANSFER_HASH_FIELDS,
        &[
            "referenceNumber",
            "amount",
            "destinationBankUUID",
            "destinationBankAccountNumber"
        ]
    );
    assert_eq!(REFUND_HASH_FIELDS, &["referenceNumber", "refundAmount"]);
    assert_eq!(
        PAYMENT_CALLBACK_HASH_FIELDS,
        &[
            "notificationId",
            "statusCode",
            "externalReferenceNumber",
            "state",
            "outstandingBalance"
        ]
    );
    assert_eq!(
        SUBSIDIARY_OPERATION_HASH_FIELDS,
        &["externalReferenceNumber", "transactionDate", "amount", "accountNumber"]
    );
    assert_eq!(
        CHARGE_MANDATE_HASH_FIELDS,
        &["referenceNumber", "amount", "accountReference"]
    );
    assert_eq!(
        DISABLE_MANDATE_HASH_FIELDS,
        &["referenceNumber", "accountReference"]
    );
}

#[test]
fn verifies_a_matching_hash_and_rejects_a_wrong_one() {
    let payload = serde_json::json!({ "referenceNumber": "REF-1" });
    let digest = compute_hash(Some(&payload), HASH_KEY, &["referenceNumber"]);
    assert!(verify_hash(Some(&payload), &digest, HASH_KEY, &["referenceNumber"]));
    assert!(!verify_hash(Some(&payload), "deadbeef", HASH_KEY, &["referenceNumber"]));
}

#[test]
fn verifies_an_uppercase_hash() {
    let payload = serde_json::json!({ "referenceNumber": "REF-1" });
    let digest = compute_hash(Some(&payload), HASH_KEY, &["referenceNumber"]);
    assert!(verify_hash(
        Some(&payload),
        &digest.to_uppercase(),
        HASH_KEY,
        &["referenceNumber"]
    ));
}

#[test]
fn verifies_a_direct_debit_mandate_callback() {
    let payload = serde_json::json!({
        "event": "Tokenization",
        "notificationId": "NOTIF-1",
        "statusCode": "004",
        "mandateStatus": "VERIFIED",
        "referenceNumber": "REF-9",
        "accountReference": "CUST-009",
        "bankName": "GTBank",
        "accountNumber": "12345678901",
        "timeStamp": "2026-11-03T11:02:25.123456789Z"
    });
    let digest = compute_hash(Some(&payload), HASH_KEY, MANDATE_CALLBACK_HASH_FIELDS);
    assert!(verify_mandate_callback(Some(&payload), &digest, HASH_KEY));
    assert!(!verify_mandate_callback(Some(&payload), "deadbeef", HASH_KEY));
}

#[test]
fn verifies_a_direct_debit_charge_callback() {
    let payload = serde_json::json!({
        "event": "Charge_Complete",
        "notificationId": "NOTIF-2",
        "statusCode": "0",
        "referenceNumber": "REF-10",
        "accountReference": "CUST-010",
        "amount": "600.00"
    });
    let digest = compute_hash(Some(&payload), HASH_KEY, CHARGE_CALLBACK_HASH_FIELDS);
    assert!(verify_charge_callback(Some(&payload), &digest, HASH_KEY));
    assert!(!verify_charge_callback(Some(&payload), "deadbeef", HASH_KEY));
}

fn sha512_hex(text: &str) -> String {
    use sha2::{Digest, Sha512};
    hex::encode(Sha512::digest(text.as_bytes()))
}
