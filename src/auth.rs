/** Authentication and hash helpers for the Paga Engine API. */

use sha2::{Digest, Sha512};
use serde_json::Value;

pub const DEFAULT_HASH_ALGORITHM: &str = "sha512";

/// Hash fields for the Dynamic virtual account "Create a payment request" operation.
pub const PAYMENT_REQUEST_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "currency",
    "payer.phoneNumber",
    "payer.email",
];

/// The longer hash field order Paga's Collect API operations page lists for the
/// payment request operation, which adds the payee fields.
pub const PAYMENT_REQUEST_FULL_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "currency",
    "payer.phoneNumber",
    "payer.email",
    "payee.accountNumber",
    "payee.phoneNumber",
    "payee.bankId",
    "payee.bankAccountNumber",
];

/// Hash fields for the Bill payment - Pay the bill operation.
pub const BILL_PAYMENT_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "merchantAccount",
    "merchantReferenceNumber",
];

/// Hash fields for the Airtime and data - Buy the airtime or data operation.
pub const AIRTIME_HASH_FIELDS: &[&str] = &["referenceNumber", "amount", "destinationPhoneNumber"];

/// Hash fields for the Money transfer - Send the money operation.
pub const MONEY_TRANSFER_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "destinationBankUUID",
    "destinationBankAccountNumber",
];

/// Hash fields for the Money transfer - Validate the account operation.
pub const VALIDATE_DEPOSIT_TO_BANK_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "destinationBankUUID",
    "destinationBankAccountNumber",
];

/// Hash fields for the Check your business account balance operation.
pub const ACCOUNT_BALANCE_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Check payment status operation.
pub const PAYMENT_STATUS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Get your transaction history operation.
pub const TRANSACTION_HISTORY_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Refunds - Refund or cancel a payment request operation.
pub const REFUND_HASH_FIELDS: &[&str] = &["referenceNumber", "refundAmount"];

/// Hash fields for the Payment callback ("Handle the callback") operation.
pub const PAYMENT_CALLBACK_HASH_FIELDS: &[&str] = &[
    "notificationId",
    "statusCode",
    "externalReferenceNumber",
    "state",
    "outstandingBalance",
];

/// Hash fields for the Subsidiary Accounts - Create an account operation.
pub const SUBSIDIARY_ACCOUNT_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "accountReference",
    "iifiNumber",
    "autoSweep.bankPublicId",
    "autoSweep.destination",
    "autoSweep.accountNumber",
    "callbackUrl",
];

/// Hash fields for the Subsidiary Accounts - Charge an account operation.
pub const SUBSIDIARY_CHARGE_HASH_FIELDS: &[&str] =
    &["referenceNumber", "amount", "currency", "narration"];

/// Hash fields for the Subsidiary Accounts - Top up an account operation.
pub const SUBSIDIARY_TOPUP_HASH_FIELDS: &[&str] =
    &["referenceNumber", "amount", "currency", "narration"];

/// Hash fields for the Subsidiary Accounts - Transfer between accounts operation.
pub const SUBSIDIARY_TRANSFER_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "sourceAccountIdentifier",
    "destinationAccountIdentifier",
    "amount",
    "currency",
    "narration",
];

/// Hash fields for the Subsidiary Accounts - Retrieve an account operation.
pub const SUBSIDIARY_RETRIEVE_HASH_FIELDS: &[&str] = &["referenceNumber", "accountIdentifier"];

/// Hash fields for the Subsidiary Accounts - Update an account operation.
pub const SUBSIDIARY_UPDATE_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "accountIdentifier",
    "iifiNumber",
    "autoSweep.destination",
    "autoSweep.bankPublicId",
    "autoSweep.accountNumber",
    "callbackUrl",
];

/// Hash fields for the Subsidiary Accounts - Get the balance operation.
pub const SUBSIDIARY_BALANCE_HASH_FIELDS: &[&str] = &["referenceNumber", "accountIdentifier"];

/// Hash fields for the Subsidiary Accounts - Get transaction history operation.
pub const SUBSIDIARY_HISTORY_HASH_FIELDS: &[&str] =
    &["accountIdentifier", "startDateTimeUTC", "endDateTimeUTC"];

/// Hash fields for the Subsidiary Accounts - Handle a funding notification operation.
pub const SUBSIDIARY_FUNDING_HASH_FIELDS: &[&str] = &[
    "transactionReference",
    "transactionDate",
    "amount",
    "accountNumber",
    "timestamp",
];

/// Hash fields for the Subsidiary Accounts - Handle a charge, top-up or transfer callback operation.
pub const SUBSIDIARY_OPERATION_HASH_FIELDS: &[&str] = &[
    "externalReferenceNumber",
    "transactionDate",
    "amount",
    "accountNumber",
];

/// Hash fields for the Checkout payment notification callback.
pub const CHECKOUT_NOTIFICATION_HASH_FIELDS: &[&str] = &["amount", "timeStamp", "paymentReference"];

/// Hash fields for the Money transfer notification callback.
pub const MONEY_TRANSFER_NOTIFICATION_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "transactionDate",
    "transactionId",
    "amount",
    "sessionId",
];

/// Hash fields for the Persistent Payment Account callback, as signed into the
/// body's `hash` field.
pub const PERSISTENT_PAYMENT_ACCOUNT_HASH_FIELDS: &[&str] = &[
    "statusCode",
    "accountNumber",
    "amount",
    "clearingFeeAmount",
    "transferFeeAmount",
];

/// Hash fields for the Persistent Payment Account callback, as signed into the
/// `x-paga-hash` header.
pub const PERSISTENT_PAYMENT_ACCOUNT_HEADER_HASH_FIELDS: &[&str] =
    &["transactionReference", "accountNumber", "amount"];

/// Hash fields for the Direct Debit - Charge the mandate operation.
pub const CHARGE_MANDATE_HASH_FIELDS: &[&str] =
    &["referenceNumber", "amount", "accountReference"];

/// Hash fields for the Direct Debit - Disable a mandate operation.
pub const DISABLE_MANDATE_HASH_FIELDS: &[&str] = &["referenceNumber", "accountReference"];

/// Hash fields for the Direct Debit - Check charge status operation.
pub const CHARGE_MANDATE_STATUS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Direct Debit - Check mandate status operation.
pub const MANDATE_STATUS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Direct Debit mandate callback.
pub const MANDATE_CALLBACK_HASH_FIELDS: &[&str] = &[
    "statusCode",
    "timeStamp",
    "accountNumber",
    "bankName",
    "referenceNumber",
    "mandateStatus",
];

/// Hash fields for the Direct Debit charge callback.
pub const CHARGE_CALLBACK_HASH_FIELDS: &[&str] = &[
    "statusCode",
    "referenceNumber",
    "accountReference",
    "amount",
];

/// Hash fields for the Bank USSD - Get the customer's bank operation.
pub const COLLECT_BANKS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Money transfer - Get the bank's UUID operation.
pub const BANKS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Airtime and data - Get the mobile operator operation.
pub const MOBILE_OPERATORS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Airtime and data - For data, pick a bundle operation.
pub const DATA_BUNDLE_HASH_FIELDS: &[&str] = &["referenceNumber", "operatorPublicId"];

/// Hash fields for the Bill payment - Find the biller operation.
pub const MERCHANTS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Bill payment - Pick the service operation.
pub const MERCHANT_SERVICES_HASH_FIELDS: &[&str] = &["referenceNumber", "merchantPublicId"];

/// Hash fields for the Bill payment - Validate the customer operation.
pub const MERCHANT_ACCOUNT_DETAILS_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "merchantAccount",
    "merchantReferenceNumber",
    "merchantServiceProductCode",
];

/// Hash fields for the Money transfer / Airtime / Bill payment - Check the
/// transaction status operation.
pub const TRANSACTION_STATUS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Business account - Get your recent transactions operation.
pub const RECENT_TRANSACTION_HISTORY_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the USD bank transfer - Check payment status operation.
pub const USD_PAYMENT_STATUS_HASH_FIELDS: &[&str] = &["referenceNumber", "paymentCurrency"];

/// Hash fields for the Collect API - Register a persistent payment account operation.
pub const REGISTER_PERSISTENT_ACCOUNT_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "accountReference",
    "financialIdentificationNumber",
    "creditBankId",
    "creditBankAccountNumber",
    "callbackUrl",
];

/// Hash fields for the Collect API - Delete a persistent payment account operation.
pub const DELETE_PERSISTENT_ACCOUNT_HASH_FIELDS: &[&str] = &["referenceNumber", "accountIdentifier"];

/// Hash fields for the Collect API - Update a persistent payment account operation.
pub const UPDATE_PERSISTENT_ACCOUNT_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "accountIdentifier",
    "financialIdentificationNumber",
    "creditBankId",
    "creditBankAccountNumber",
    "callbackUrl",
];

/// Hash fields for the Collect API - Get a persistent payment account operation.
pub const GET_PERSISTENT_ACCOUNT_HASH_FIELDS: &[&str] = &["referenceNumber", "accountIdentifier"];

/// Hash fields for the Collect API - Payment request history operation.
pub const HISTORY_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Business API - Register a customer operation.
pub const CUSTOMER_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "customerPhoneNumber",
    "customerFirstName",
    "customerLastName",
];

/// Hash fields for the Business API - Register a customer account photo operation.
pub const CUSTOMER_ACCOUNT_PHOTO_HASH_FIELDS: &[&str] = &["referenceNumber", "customerPhoneNumber"];

/// Hash fields for the Business API - Register a customer identification operation.
pub const CUSTOMER_IDENTIFICATION_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "customerPhoneNumber",
    "customerIdType",
    "customerIdNumber",
    "customerIdExpirationDate",
];

/// Hash fields for the Business API - Money transfer to a Paga wallet operation.
pub const PAGA_WALLET_TRANSFER_HASH_FIELDS: &[&str] =
    &["referenceNumber", "amount", "destinationAccount"];

/// Hash fields for the Business API - Validate a money transfer operation.
pub const VALIDATE_MONEY_TRANSFER_HASH_FIELDS: &[&str] =
    &["referenceNumber", "amount", "destinationAccount"];

/// Hash fields for the Business API - Bulk money transfer operation.
pub const MONEY_TRANSFER_BULK_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "amount",
    "destinationAccount",
    "itemCount",
];

/// Hash fields for the Business API - Get an operation's status operation.
pub const GET_OPERATION_STATUS_HASH_FIELDS: &[&str] = &["referenceNumber"];

/// Hash fields for the Business API - Onboard a merchant operation.
pub const ONBOARD_MERCHANT_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "merchantExternalId",
    "name",
    "phone",
    "email",
];

/// Hash fields for the Business API - Dispense cash operation.
pub const DISPENSE_CASH_HASH_FIELDS: &[&str] = &[
    "referenceNumber",
    "customerPhoneNumber",
    "amount",
    "withdrawalCode",
];

/// Resolve a possibly dotted field path on a payload.
fn resolve_value<'a>(payload: Option<&'a Value>, path: &str) -> Option<&'a Value> {
    let mut node = payload?;
    for key in path.split('.') {
        node = node.as_object()?.get(key)?;
    }
    Some(node)
}

/// Render a resolved hash field exactly as Paga signs it.
///
/// Strings are used verbatim — no quotes are added. Numbers are rendered as
/// they are sent, except for money fields carried as numbers, which Paga
/// signs with two decimals and no separators.
fn field_to_text(value: Option<&Value>, field: &str) -> String {
    match value {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Number(n)) => {
            if field.ends_with("Balance") {
                match n.as_f64() {
                    Some(f) => format!("{:.2}", f),
                    None => n.to_string(),
                }
            } else {
                n.to_string()
            }
        }
        Some(other) => other.to_string(),
    }
}

/// Compute the Paga request hash.
///
/// The hash is the SHA-512 hex digest of the concatenated hash field values
/// (in field order) followed by the business hash key.
pub fn compute_hash(payload: Option<&Value>, hash_key: &str, fields: &[&str]) -> String {
    let mut text = String::new();
    for field in fields {
        text.push_str(&field_to_text(resolve_value(payload, field), field));
    }
    text.push_str(hash_key);
    let digest = Sha512::digest(text.as_bytes());
    hex::encode(digest)
}

/// Compare two strings in constant time.
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.as_bytes().iter().zip(b.as_bytes()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Verify a Paga hash.
///
/// The comparison is case-insensitive and constant time.
pub fn verify_hash(payload: Option<&Value>, received_hash: &str, hash_key: &str, fields: &[&str]) -> bool {
    let computed = compute_hash(payload, hash_key, fields);
    constant_time_eq(&computed, &received_hash.trim().to_lowercase())
}

/// Verify a Paga hash that arrived in a header rather than the body.
///
/// Paga's `x-paga-hash-parameters` header names the fields it signed, in
/// order. When that header is present it wins over the caller's default
/// order.
pub fn verify_hash_header(
    payload: Option<&Value>,
    received_hash: &str,
    hash_key: &str,
    fields: &[&str],
    hash_parameters: Option<&[String]>,
) -> bool {
    match hash_parameters {
        Some(parameters) if !parameters.is_empty() => {
            let mut text = String::new();
            for field in parameters {
                text.push_str(&field_to_text(
                    resolve_value(payload, field),
                    field,
                ));
            }
            text.push_str(hash_key);
            let digest = Sha512::digest(text.as_bytes());
            constant_time_eq(&hex::encode(digest), &received_hash.trim().to_lowercase())
        }
        _ => verify_hash(payload, received_hash, hash_key, fields),
    }
}

/// Parse Paga's `x-paga-hash-parameters` header.
///
/// A trailing `hashKey` entry is dropped: the SDK appends the key itself.
pub fn parse_hash_parameters(raw: &str) -> Vec<String> {
    raw.replace(';', ",")
        .split(',')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .filter(|part| !part.eq_ignore_ascii_case("hashkey"))
        .map(|part| part.to_string())
        .collect()
}
