# paga-rs

**Async Rust SDK for the [Paga Engine API](https://developer-docs.paga.com/).**

`paga-rs` makes the Paga developer workflow easy for page and integration
developers building checkout and payment experiences. It wraps the Paga Engine
API — payments, refunds, money transfers, airtime, bill payments, Direct Debit,
subsidiary accounts and business account operations — behind a single, fully
async client.

The Python (`paga-python`) and TypeScript (`paga-ts`) counterparts live in the
same organisation and track the same operation set.

## Features

- **Async all the way through** — built on `reqwest` and `tokio`.
- **Simple interface** — every operation takes plain arguments or a request
  struct; no raw JSON payloads needed.
- **Built-in Paga hash signing** — SHA-512 request hashing with the same hash field
  order as the other SDKs, plus webhook signature verification helpers.
- **Typed responses** — serde models for every endpoint response.
- **Typed errors** — `PagaError`, `PagaAuthenticationError`, `PagaAPIError`,
  `PagaRateLimitError`.
- **Typed status codes** — `CollectStatusCode` and `BusinessErrorCode`, plus
  `classify()` to turn a response into a `Success` / `Pending` / `Partial` /
  `Failed` / `Unknown` outcome.
- **Unknown-outcome detection** — Paga's `-1`, `105` and `135` mean *unknown*, not
  *failed*. `PagaError::is_unknown_outcome` tells you when to check the transaction
  status instead of resending.

> Paga requires your server's IP address to be whitelisted before any call will
> succeed. A 403 usually means the IP is not on the list.

## Installation

```bash
cargo add paga
```

## Quick start

```rust,no_run
use paga::{CreatePayment, PagaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PagaClient::new(
        "YOUR_PUBLIC_KEY",
        "YOUR_SECRET_KEY",
        "YOUR_HASH_KEY",
        "test",
    )?;

    let payment = client
        .create_payment(CreatePayment::new(
            "ORD-2026-0187",
            50.0,
            "Bello Ramon",
            "08064343187",
            "Your business name",
        ))
        .await?;

    let status = client.get_payment_status("ORD-2026-0187").await?;

    Ok(())
}
```

Operations with optional fields take a request struct. The required fields are
set by `new`; the optional ones are public, so they can be set with
struct-update syntax:

```rust,no_run
# use paga::{CreatePayment, PagaClient};
# async fn run(client: PagaClient) -> Result<(), paga::PagaError> {
let request = CreatePayment::new("ORD-1", 50.0, "Bello", "0801234", "Payee");
let request = CreatePayment {
    payer_email: Some("customer@example.com".to_string()),
    call_back_url: Some("https://yourstore.com/webhooks/paga".to_string()),
    ..request
};
client.create_payment(request).await?;
# Ok(())
# }
```

## Supported operations

| Area | Operation | Method |
|---|---|---|
| Checkout | Create payment request | `create_payment` |
| Checkout | Check payment status | `get_payment_status` |
| Checkout | Verify charge | `verify_charge` |
| Refunds | Refund / cancel | `refund_payment` |
| Refunds | Check refund status | `get_refund_status` |
| Bank USSD | List customer's banks | `get_collect_banks` |
| Direct Debit | Start hosted set-up session | `start_direct_debit_session` |
| Direct Debit | Set up a mandate by API | `create_direct_debit_mandate` |
| Direct Debit | Check mandate status | `get_mandate_status` |
| Direct Debit | Charge the mandate | `charge_mandate` |
| Direct Debit | Check charge status | `get_charge_mandate_status` |
| Direct Debit | Disable a mandate | `disable_mandate` |
| USD collection | Check payment status | `get_usd_payment_status` |
| Money transfer | List institutions | `get_banks` |
| Money transfer | Validate the account | `validate_deposit_to_bank` |
| Money transfer | Send money to a bank account | `send_money` |
| Money transfer | Send money to a Paga wallet | `transfer_to_paga_wallet` |
| Money transfer | Validate a Paga wallet transfer | `validate_money_transfer` |
| Money transfer | Bulk money transfer | `send_bulk_money_transfer` |
| Money transfer | Check transaction status | `get_transaction_status` |
| Airtime & data | List operators | `get_mobile_operators` |
| Airtime & data | List data bundles | `get_data_bundles` |
| Airtime & data | Buy airtime / data | `buy_airtime` |
| Bill payment | List billers | `get_merchants` |
| Bill payment | List a biller's services | `get_merchant_services` |
| Bill payment | Validate the customer | `get_merchant_account_details` |
| Bill payment | Pay bill | `pay_bill` |
| Business | Check balance | `get_business_balance` |
| Business | Transaction history | `get_business_transactions` |
| Business | Recent transactions | `get_recent_transactions` |
| Business | Operation status | `get_operation_status` |
| Business | Onboard a merchant | `onboard_merchant` |
| Business | Dispense cash | `dispense_cash` |
| Subsidiary accounts | Create | `create_subsidiary_account` |
| Subsidiary accounts | Charge | `charge_subsidiary_account` |
| Subsidiary accounts | Top up | `top_up_subsidiary_account` |
| Subsidiary accounts | Transfer between accounts | `transfer_between_subsidiary_accounts` |
| Subsidiary accounts | Retrieve | `retrieve_subsidiary_account` |
| Subsidiary accounts | Update | `update_subsidiary_account` |
| Subsidiary accounts | Balance | `get_subsidiary_balance` |
| Subsidiary accounts | History | `get_subsidiary_history` |
| Persistent accounts | Register | `register_persistent_payment_account` |
| Persistent accounts | Update | `update_persistent_payment_account` |
| Persistent accounts | Get | `get_persistent_payment_account` |
| Persistent accounts | Delete | `delete_persistent_payment_account` |
| Payment history | Request history | `get_payment_request_history` |
| Customers | Register | `register_customer` |
| Customers | Register with KYC | `register_customer_with_kyc` |
| Customers | Upload account photo | `register_customer_account_photo` |
| Customers | Upload identification | `register_customer_identification` |

## Webhook verification

```rust,no_run
use paga::webhooks;

# let payload = serde_json::json!({});
# let received_hash = "";
# let hash_parameters: Option<&str> = None;

let ok = webhooks::verify_checkout_notification(Some(&payload), &received_hash, "YOUR_HASH_KEY");
let ok = webhooks::verify_payment_callback(Some(&payload), &received_hash, "YOUR_HASH_KEY");
let ok = webhooks::verify_funding_notification(Some(&payload), &received_hash, "YOUR_HASH_KEY");
let ok = webhooks::verify_subsidiary_operation(Some(&payload), &received_hash, "YOUR_HASH_KEY");

// The money transfer digest travels in the x-paga-hash header, not the body.
let ok = webhooks::verify_money_transfer_notification(
    Some(&payload),
    &received_hash,
    "YOUR_HASH_KEY",
    hash_parameters,
);

// Direct Debit callbacks.
let ok = webhooks::verify_mandate_callback(Some(&payload), &received_hash, "YOUR_HASH_KEY");
let ok = webhooks::verify_charge_callback(Some(&payload), &received_hash, "YOUR_HASH_KEY");
```

The USD (ACH, Wire and Swift) webhook carries **no signature**, so there is
nothing to verify — parse it and de-duplicate on `referenceNumber` plus
`paymentId`:

```rust,no_run
# let payload = serde_json::json!({});
let webhook = paga::webhooks::parse_usd_payment_webhook(Some(&payload));
if webhook.is_captured() {
    // fulfil
}
```

## Error handling

```rust,no_run
# use paga::PagaClient;
# async fn run(client: PagaClient) -> Result<(), paga::PagaError> {
match client.get_banks("REF-1").await {
    Ok(banks) => { let _ = banks; }
    Err(error) if error.kind == paga::PagaErrorKind::Authentication => { /* bad keys */ }
    Err(error) if error.kind == paga::PagaErrorKind::RateLimit => { /* 429 */ }
    Err(error) => {
        // error.code, error.outcome, error.status_code, error.response_code
        let _ = error;
    }
}
# Ok(())
# }
```

A 200 response does not mean the money moved. Read the code in the body:

```rust,no_run
# use paga::PagaClient;
# async fn run(client: PagaClient) -> Result<(), paga::PagaError> {
let status = client
    .get_transaction_status(paga::GetTransactionStatus::new("PAY-2026-0311"))
    .await?;

let body = serde_json::to_value(&status).unwrap();
if paga::classify(Some(&body)) == paga::OperationOutcome::Success {
    // settled
}
# Ok(())
# }
```

When the outcome is **unknown** — Paga's `-1`, `105` and `135`, a timeout or a
504 — do not send again. Check the transaction status with the *original*
reference number.

## License

GPL-3.0 — see [LICENSE](LICENSE).
