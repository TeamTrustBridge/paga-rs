# paga-rs

**Async Rust SDK for the [Paga Engine API](https://developer-docs.paga.com/).**

`paga-rs` makes the Paga developer workflow easy for page and integration
developers building checkout and payment experiences. It wraps every Paga Engine
endpoint — payments, refunds, money transfers, airtime, bill payments, subsidiary
accounts, and business account operations — behind a single, fully async client.

The Python (`paga-python`) and TypeScript (`paga-ts`) counterparts are in the pipeline.

## Features

- **Async all the way through** — built on `reqwest` and `tokio`.
- **Simple interface** — every operation takes plain arguments; no raw JSON payloads needed.
- **Built-in Paga hash signing** — SHA-512 request hashing with the same hash field
  order as the other SDKs, plus webhook signature verification helpers.
- **Typed responses** — serde models for every endpoint response.
- **Typed errors** — `PagaError`, `PagaAuthenticationError`, `PagaAPIError`,
  `PagaRateLimitError`.

## Installation

```bash
cargo add paga
```

## Quick start

```rust
use paga::{PagaClient, PagaClient as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PagaClient::new(
        "YOUR_PUBLIC_KEY",
        "YOUR_SECRET_KEY",
        "YOUR_HASH_KEY",
        "test",
    );

    let payment = client.create_payment("ORD-2026-0187", 50.0, "Bello Ramon", "08064343187", "Your business name").await?;
    let status = client.get_payment_status("ORD-2026-0187").await?;

    Ok(())
}
```

## Supported operations

| Area | Operation | Method |
|---|---|---|
| Checkout | Create payment request / Check status / Verify charge | `create_payment`, `get_payment_status`, `verify_charge` |
| Refunds | Refund / cancel | `refund_payment` |
| Money transfer | Send money | `send_money` |
| Airtime & data | Buy airtime / data | `buy_airtime` |
| Bill payment | Pay bill | `pay_bill` |
| Business | Check balance / History | `get_business_balance`, `get_business_transactions` |
| Subsidiary accounts | Create / charge / top-up / transfer / retrieve / update / balance / history | `create_subsidiary_account`, `charge_subsidiary_account`, `top_up_subsidiary_account`, `transfer_between_subsidiary_accounts`, `retrieve_subsidiary_account`, `update_subsidiary_account`, `get_subsidiary_balance`, `get_subsidiary_history` |

## Webhook verification

```rust
use paga::webhooks;

let ok = webhooks::verify_checkout_notification(Some(&payload), &received_hash, "YOUR_HASH_KEY");
```

## License

GPL-3.0 — see [LICENSE](LICENSE).
