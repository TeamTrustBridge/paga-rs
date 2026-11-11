//! Request shape tests: URL, auth style, hash header and body.

mod support;

use paga::auth::compute_hash;
use paga::requests::*;
use paga::{Environment, PagaClient, PagaClientOptions};
use support::MockServer;

const PUBLIC_KEY: &str = "PUBLIC-KEY";
const SECRET_KEY: &str = "SECRET-KEY";
const HASH_KEY: &str = "hashkey123";

async fn client_for(server: &MockServer) -> PagaClient {
    let base = server.base_url().to_string();
    PagaClient::with_options(
        PUBLIC_KEY,
        SECRET_KEY,
        HASH_KEY,
        PagaClientOptions {
            environment: Environment::Test,
            base_url: Some(base.clone()),
            checkout_base_url: Some(base.clone()),
            business_base_url: Some(base),
            ..Default::default()
        },
    )
}

#[tokio::test]
async fn collect_lookups_use_basic_auth() {
    let server = MockServer::start(200, r#"{"statusCode":"0","banks":[]}"#).await;
    let client = client_for(&server).await;

    client.get_collect_banks("REF-1").await.expect("request");

    let request = server.last();
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/banks");
    assert_eq!(
        request.basic_credentials(),
        Some((PUBLIC_KEY.to_string(), SECRET_KEY.to_string()))
    );
    assert!(request.header("hash").is_some());
}

#[tokio::test]
async fn business_lookups_use_header_auth() {
    let server = MockServer::start(200, r#"{"responseCode":0,"banks":[]}"#).await;
    let client = client_for(&server).await;

    client.get_banks("REF-2").await.expect("request");

    let request = server.last();
    assert_eq!(request.path, "/getBanks");
    assert_eq!(request.header("principal"), Some(PUBLIC_KEY));
    assert_eq!(request.header("credentials"), Some(SECRET_KEY));
    assert!(request.header("authorization").is_none());
    assert!(request.header("hash").is_some());
}

#[tokio::test]
async fn the_hash_covers_the_documented_fields() {
    let server = MockServer::start(200, r#"{"responseCode":0}"#).await;
    let client = client_for(&server).await;

    client
        .get_merchant_services("REF-3", "MERCHANT-UUID")
        .await
        .expect("request");

    let request = server.last();
    let expected = compute_hash(
        Some(&request.json()),
        HASH_KEY,
        &["referenceNumber", "merchantPublicId"],
    );
    assert_eq!(request.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn the_merchant_service_product_code_is_always_signed() {
    let server = MockServer::start(200, r#"{"responseCode":0}"#).await;
    let client = client_for(&server).await;

    client
        .get_merchant_account_details("REF-4", "MERCHANT-UUID", "8110411076", None)
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.json()["merchantServiceProductCode"], "");
    let expected = compute_hash(
        Some(&request.json()),
        HASH_KEY,
        &[
            "referenceNumber",
            "merchantAccount",
            "merchantReferenceNumber",
            "merchantServiceProductCode",
        ],
    );
    assert_eq!(request.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn verify_charge_sends_its_body() {
    // The body used to be dropped whenever the request was not hashed.
    let server = MockServer::start(200, r#"{"status_code":0,"chargeId":"CHARGE-1"}"#).await;
    let client = client_for(&server).await;

    let response = client
        .verify_charge("PAY-1", PUBLIC_KEY, 50.0)
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/checkout/transaction/verify");
    assert_eq!(request.json()["paymentReference"], "PAY-1");
    assert_eq!(request.json()["amount"], 50.0);
    assert_eq!(response.charge_id.as_deref(), Some("CHARGE-1"));
}

#[tokio::test]
async fn verify_charge_does_not_raise_on_a_paid_charge() {
    // A paid charge is `status_code` 0; it used to be reported as a failure.
    let server = MockServer::start(200, r#"{"status_code":0}"#).await;
    let client = client_for(&server).await;

    let response = client
        .verify_charge("PAY-1", PUBLIC_KEY, 50.0)
        .await
        .expect("a paid charge must not raise");
    assert_eq!(response.status_code, Some(0));
}

#[tokio::test]
async fn the_transaction_status_is_read_from_the_business_api() {
    let server = MockServer::start(200, r#"{"responseCode":0,"transactionId":"TX-1"}"#).await;
    let client = client_for(&server).await;

    let response = client
        .get_transaction_status(GetTransactionStatus::new("REF-6"))
        .await
        .expect("request");

    assert_eq!(server.last().path, "/transactionStatus");
    assert_eq!(response.transaction_id.as_deref(), Some("TX-1"));
}

#[tokio::test]
async fn the_usd_status_signs_the_currency() {
    let server = MockServer::start(200, r#"{"statusCode":"0"}"#).await;
    let client = client_for(&server).await;

    client
        .get_usd_payment_status("REF-8", "USD")
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.json()["paymentCurrency"], "USD");
    let expected = compute_hash(
        Some(&request.json()),
        HASH_KEY,
        &["referenceNumber", "paymentCurrency"],
    );
    assert_eq!(request.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn a_direct_debit_charge_signs_three_fields() {
    let server = MockServer::start(200, r#"{"statusCode":"0"}"#).await;
    let client = client_for(&server).await;

    client
        .charge_mandate(ChargeMandate::new("REF-6", 600.0, "CUST-006"))
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/chargeDebitMandate");
    let expected = compute_hash(
        Some(&request.json()),
        HASH_KEY,
        &["referenceNumber", "amount", "accountReference"],
    );
    assert_eq!(request.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn a_hosted_direct_debit_session_sends_no_hash_header() {
    let server = MockServer::start(200, r#"{"statusCode":"0","data":{}}"#).await;
    let client = client_for(&server).await;

    client
        .start_direct_debit_session(StartDirectDebitSession::new(
            "REF-1",
            "CUST-001",
            50_000.0,
            "https://yourstore.com/webhooks/mandate",
        ))
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/checkout/charge-process/start");
    assert!(request.header("hash").is_none());
    assert!(request.basic_credentials().is_some());
    assert_eq!(request.json()["paymentMethods"][0], "BANK_ACCOUNT_DIRECT_DEBIT");
}

#[tokio::test]
async fn a_mandate_is_created_with_the_direct_debit_payment_method() {
    let server = MockServer::start(200, r#"{"statusCode":"0"}"#).await;
    let client = client_for(&server).await;

    client
        .create_direct_debit_mandate(CreateDirectDebitMandate::new(
            "REF-3",
            5000.0,
            "Bello Ramon",
            "08064343187",
            "Your business name",
            "CUST-003",
        ))
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/paymentRequest");
    assert_eq!(request.json()["paymentMethods"][0], "DIRECT_DEBIT");
    assert_eq!(request.json()["accountReference"], "CUST-003");
    let expected = compute_hash(
        Some(&request.json()),
        HASH_KEY,
        &[
            "referenceNumber",
            "amount",
            "currency",
            "payer.phoneNumber",
            "payer.email",
        ],
    );
    assert_eq!(request.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn a_mandate_status_reads_the_process_status_id() {
    let server = MockServer::start(
        200,
        r#"{"statusCode":"0","data":{"statusCode":"1","additionalData":{"mandate":[{"ProcessStatusId":"APPROVED"}]}}}"#,
    )
    .await;
    let client = client_for(&server).await;

    let response = client
        .get_mandate_status("REF-4")
        .await
        .expect("request");

    assert_eq!(response.mandate_status(), Some("APPROVED"));
    assert!(response.is_approved());
}

#[tokio::test]
async fn retrieving_a_subsidiary_account_sends_the_reference_in_the_query() {
    // The reference used to be dropped from the query string.
    let server = MockServer::start(200, r#"{"statusCode":"0"}"#).await;
    let client = client_for(&server).await;

    client
        .retrieve_subsidiary_account("CUST-1", "REF-1")
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/subsidiary-accounts/CUST-1");
    assert!(request
        .query
        .as_deref()
        .unwrap_or_default()
        .contains("referenceNumber=REF-1"));
}

#[tokio::test]
async fn a_subsidiary_history_sends_every_query_parameter() {
    // The page parameters used to be dropped.
    let server = MockServer::start(200, r#"{"statusCode":"0"}"#).await;
    let client = client_for(&server).await;

    let mut request = GetSubsidiaryHistory::new(
        "CUST-1",
        "REF-1",
        "2026-06-01T00:00:00",
        "2026-06-30T23:59:59",
    );
    request.page_number = Some(2);
    request.page_size = Some(50);

    client.get_subsidiary_history(request).await.expect("request");

    let recorded = server.last();
    assert_eq!(recorded.path, "/subsidiary-accounts/CUST-1/history");
    let query = recorded.query.clone().unwrap_or_default();
    assert!(query.contains("referenceNumber=REF-1"), "{}", query);
    assert!(query.contains("pageNumber=2"), "{}", query);
    assert!(query.contains("pageSize=50"), "{}", query);

    let expected = compute_hash(
        Some(&recorded.json()),
        HASH_KEY,
        &["accountIdentifier", "startDateTimeUTC", "endDateTimeUTC"],
    );
    assert_eq!(recorded.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn a_bulk_transfer_signs_the_first_item_and_the_count() {
    let server = MockServer::start(200, r#"{"responseCode":0,"results":[]}"#).await;
    let client = client_for(&server).await;

    client
        .send_bulk_money_transfer(SendBulkMoneyTransfer::new(vec![
            BulkTransferItem::new("ITEM-1", 100.0, "08031234567"),
            BulkTransferItem::new("ITEM-2", 200.0, "08031234568"),
        ]))
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/moneyTransferBulk");
    assert_eq!(request.json()["items"].as_array().unwrap().len(), 2);

    let expected = compute_hash(
        Some(&serde_json::json!({
            "referenceNumber": "ITEM-1",
            "amount": 100.0,
            "destinationAccount": "08031234567",
            "itemCount": 2,
        })),
        HASH_KEY,
        &["referenceNumber", "amount", "destinationAccount", "itemCount"],
    );
    assert_eq!(request.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn a_bulk_transfer_over_three_hundred_items_is_rejected() {
    let server = MockServer::start(200, "{}").await;
    let client = client_for(&server).await;

    let items: Vec<BulkTransferItem> = (0..301)
        .map(|index| BulkTransferItem::new(format!("ITEM-{}", index), 1.0, "080"))
        .collect();
    let result = client.send_bulk_money_transfer(SendBulkMoneyTransfer::new(items)).await;

    assert!(result.is_err());
    assert_eq!(server.count(), 0);
}

#[tokio::test]
async fn a_kyc_registration_sends_multipart() {
    let server = MockServer::start(200, r#"{"responseCode":0}"#).await;
    let client = client_for(&server).await;

    client
        .register_customer_with_kyc(RegisterCustomerWithKyc::new(
            "REF-5",
            "08031234567",
            "customer@example.com",
            "Bello",
            "Ramon",
            "1989-10-02",
            "MALE",
            b"account-photo-bytes".to_vec(),
            b"id-photo-bytes".to_vec(),
        ))
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/registerCustomerWithKYC");
    let content_type = request.header("content-type").unwrap_or_default();
    assert!(content_type.starts_with("multipart/form-data"), "{}", content_type);
    assert!(request.body.contains("customerAccountPhoto"));
    assert!(request.body.contains("account-photo-bytes"));
}

#[tokio::test]
async fn a_customer_registration_posts_json() {
    let server = MockServer::start(200, r#"{"responseCode":0}"#).await;
    let client = client_for(&server).await;

    client
        .register_customer(RegisterCustomer::new(
            "REF-4",
            "08031234567",
            "Bello",
            "Ramon",
            "customer@example.com",
            "1989-10-02",
        ))
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/registerCustomer");
    assert_eq!(request.json()["customerEmail"], "customer@example.com");
    let expected = compute_hash(
        Some(&request.json()),
        HASH_KEY,
        &[
            "referenceNumber",
            "customerPhoneNumber",
            "customerFirstName",
            "customerLastName",
        ],
    );
    assert_eq!(request.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn a_payment_request_signs_the_documented_fields() {
    let server = MockServer::start(200, r#"{"statusCode":"0"}"#).await;
    let client = client_for(&server).await;

    client
        .create_payment(CreatePayment::new(
            "REF-14",
            50.0,
            "Bello Ramon",
            "08064343187",
            "Your business name",
        ))
        .await
        .expect("request");

    let request = server.last();
    assert_eq!(request.path, "/paymentRequest");
    let expected = compute_hash(
        Some(&request.json()),
        HASH_KEY,
        &[
            "referenceNumber",
            "amount",
            "currency",
            "payer.phoneNumber",
            "payer.email",
        ],
    );
    assert_eq!(request.header("hash"), Some(expected.as_str()));
}

#[tokio::test]
async fn a_collect_success_is_not_treated_as_a_failure() {
    // Collect reports success in `statusCode`; only `responseCode` was
    // checked, so every Collect call used to raise.
    let server = MockServer::start(200, r#"{"statusCode":"0","referenceNumber":"REF-1"}"#).await;
    let client = client_for(&server).await;

    let response = client
        .create_payment(CreatePayment::new(
            "REF-1",
            50.0,
            "Bello Ramon",
            "08064343187",
            "Your business name",
        ))
        .await
        .expect("a successful Collect call must not raise");

    assert_eq!(response.reference_number.as_deref(), Some("REF-1"));
}

#[tokio::test]
async fn a_business_success_is_not_treated_as_a_failure() {
    let server = MockServer::start(200, r#"{"responseCode":0,"transactionId":"TX-1"}"#).await;
    let client = client_for(&server).await;

    let response = client
        .get_business_balance("REF-1")
        .await
        .expect("request");

    assert_eq!(response.response_code, Some(0));
}

#[tokio::test]
async fn an_authentication_failure_raises() {
    let server = MockServer::start(401, r#"{"error":"Invalid paga account id or password"}"#).await;
    let client = client_for(&server).await;

    let result = client.get_banks("REF-1").await;

    let error = result.expect_err("a 401 must raise");
    assert_eq!(error.kind, paga::PagaErrorKind::Authentication);
}

#[tokio::test]
async fn a_rate_limited_request_is_not_retried_by_default() {
    let server = MockServer::start(429, r#"{"message":"slow down"}"#).await;
    let client = client_for(&server).await;

    let result = client.get_banks("REF-1").await;

    assert!(result.is_err());
    assert_eq!(server.count(), 1);
}
