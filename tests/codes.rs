//! Status code and outcome classification tests.

use paga_rs::codes::{
    classify, is_unknown_outcome, resolve_business_code, resolve_collect_status,
    BusinessErrorCode, ChargeStatusCode, CollectStatusCode, MandateProcessStatus,
    MandateStatusCode, OperationOutcome,
};
use serde_json::json;

#[test]
fn resolves_the_collect_status_codes() {
    assert_eq!(
        resolve_collect_status(Some(&json!({ "statusCode": "0" }))),
        Some(CollectStatusCode::Success)
    );
    assert_eq!(
        resolve_collect_status(Some(&json!({ "statusCode": "1" }))),
        Some(CollectStatusCode::InProcess)
    );
    assert_eq!(
        resolve_collect_status(Some(&json!({ "statusCode": "2" }))),
        Some(CollectStatusCode::PartialPayment)
    );
    assert_eq!(
        resolve_collect_status(Some(&json!({ "statusCode": "-1" }))),
        Some(CollectStatusCode::SystemError)
    );
    assert_eq!(
        resolve_collect_status(Some(&json!({ "statusCode": "114" }))),
        Some(CollectStatusCode::Failed)
    );
    assert_eq!(
        resolve_collect_status(Some(&json!({ "statusCode": "121" }))),
        Some(CollectStatusCode::NotFound)
    );
}

#[test]
fn accepts_an_integer_status_code() {
    assert_eq!(
        resolve_collect_status(Some(&json!({ "statusCode": 0 }))),
        Some(CollectStatusCode::Success)
    );
}

#[test]
fn returns_none_for_an_unrecognised_status_code() {
    assert_eq!(resolve_collect_status(Some(&json!({ "statusCode": "999" }))), None);
}

#[test]
fn resolves_the_business_error_codes() {
    assert_eq!(
        resolve_business_code(Some(&json!({ "responseCode": 0 }))),
        Some(BusinessErrorCode::Success)
    );
    assert_eq!(
        resolve_business_code(Some(&json!({ "responseCode": 3 }))),
        Some(BusinessErrorCode::Pending)
    );
    assert_eq!(
        resolve_business_code(Some(&json!({ "responseCode": -1 }))),
        Some(BusinessErrorCode::UnknownError)
    );
    assert_eq!(
        resolve_business_code(Some(&json!({ "responseCode": 105 }))),
        Some(BusinessErrorCode::DuplicateReference)
    );
    assert_eq!(
        resolve_business_code(Some(&json!({ "responseCode": 135 }))),
        Some(BusinessErrorCode::UnknownReferenceNumber)
    );
    assert_eq!(
        resolve_business_code(Some(&json!({ "responseCode": 139 }))),
        Some(BusinessErrorCode::InsufficientBalance)
    );
}

#[test]
fn classifies_the_collect_envelope() {
    assert_eq!(
        classify(Some(&json!({ "statusCode": "0" }))),
        OperationOutcome::Success
    );
    assert_eq!(
        classify(Some(&json!({ "statusCode": "1" }))),
        OperationOutcome::Pending
    );
    assert_eq!(
        classify(Some(&json!({ "statusCode": "2" }))),
        OperationOutcome::Partial
    );
    assert_eq!(
        classify(Some(&json!({ "statusCode": "-1" }))),
        OperationOutcome::Unknown
    );
    assert_eq!(
        classify(Some(&json!({ "statusCode": "-2" }))),
        OperationOutcome::Failed
    );
    assert_eq!(
        classify(Some(&json!({ "statusCode": "114" }))),
        OperationOutcome::Failed
    );
}

#[test]
fn classifies_the_business_envelope() {
    assert_eq!(
        classify(Some(&json!({ "responseCode": 0 }))),
        OperationOutcome::Success
    );
    assert_eq!(
        classify(Some(&json!({ "responseCode": 1 }))),
        OperationOutcome::Success
    );
    assert_eq!(
        classify(Some(&json!({ "responseCode": 3 }))),
        OperationOutcome::Pending
    );
    assert_eq!(
        classify(Some(&json!({ "responseCode": -1 }))),
        OperationOutcome::Unknown
    );
    assert_eq!(
        classify(Some(&json!({ "responseCode": 105 }))),
        OperationOutcome::Unknown
    );
    assert_eq!(
        classify(Some(&json!({ "responseCode": 135 }))),
        OperationOutcome::Unknown
    );
    assert_eq!(
        classify(Some(&json!({ "responseCode": 139 }))),
        OperationOutcome::Failed
    );
}

#[test]
fn treats_a_body_with_no_code_as_unknown() {
    assert_eq!(
        classify(Some(&json!({ "message": "ok" }))),
        OperationOutcome::Unknown
    );
    assert_eq!(classify(None), OperationOutcome::Unknown);
}

#[test]
fn flags_the_unknown_outcome_codes() {
    assert!(is_unknown_outcome(Some(&json!({ "responseCode": -1 }))));
    assert!(is_unknown_outcome(Some(&json!({ "responseCode": 105 }))));
    assert!(is_unknown_outcome(Some(&json!({ "responseCode": 135 }))));
    assert!(!is_unknown_outcome(Some(&json!({ "responseCode": 139 }))));
}

#[test]
fn exposes_the_direct_debit_vocabularies() {
    assert_eq!(MandateStatusCode::Approved.as_str(), "0");
    assert_eq!(MandateStatusCode::Verified.as_str(), "004");
    assert_eq!(MandateProcessStatus::Approved.as_str(), "APPROVED");
    assert_eq!(ChargeStatusCode::Charged.as_i64(), 0);
    assert_eq!(ChargeStatusCode::Invalid.as_i64(), -1);
    assert_eq!(ChargeStatusCode::Unknown.as_i64(), -4);
}
