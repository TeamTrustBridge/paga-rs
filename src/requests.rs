/** Request types for the Paga Engine API operations.
 *
 * Every operation that has optional fields takes a request struct. The
 * required fields are set by `new`; the optional ones are public, so they
 * can be set with struct-update syntax:
 *
 * ```ignore
 * let request = CreatePayment::new("ORD-1", 50.0, "Bello", "0801234", "Payee")
 *     .payer_email("customer@example.com".to_string());
 * ```
 */

use serde_json::{json, Map, Value};

fn obj(pairs: Vec<(&str, Value)>) -> Value {
    let mut map = Map::new();
    for (key, value) in pairs {
        map.insert(key.to_string(), value);
    }
    Value::Object(map)
}

fn insert_opt(target: &mut Map<String, Value>, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        target.insert(key.to_string(), json!(value));
    }
}

/// Create a payment request.
#[derive(Debug, Clone, Default)]
pub struct CreatePayment {
    pub reference_number: String,
    pub amount: f64,
    pub payer_name: String,
    pub payer_phone_number: String,
    pub payee_name: String,
    pub currency: Option<String>,
    pub payer_email: Option<String>,
    pub call_back_url: Option<String>,
    pub payment_methods: Option<Vec<String>>,
    pub expiry_date_time_utc: Option<String>,
    pub is_suppress_messages: bool,
    pub payer_collection_fee_share: Option<f64>,
    pub payee_collection_fee_share: Option<f64>,
    pub payer_address: Option<String>,
    pub payer_bank_id: Option<String>,
    pub payer_bank_account_number: Option<String>,
    pub payer_currency: Option<String>,
    pub payee_account_number: Option<String>,
    pub payee_phone_number: Option<String>,
    pub payee_bank_id: Option<String>,
    pub payee_bank_account_number: Option<String>,
    pub is_single_use: Option<bool>,
    pub is_allow_partial_payments: Option<bool>,
    pub is_allow_over_payments: Option<bool>,
    pub narration: Option<String>,
    pub redirect_url: Option<String>,
}

impl CreatePayment {
    pub fn new(
        reference_number: impl Into<String>,
        amount: f64,
        payer_name: impl Into<String>,
        payer_phone_number: impl Into<String>,
        payee_name: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount,
            payer_name: payer_name.into(),
            payer_phone_number: payer_phone_number.into(),
            payee_name: payee_name.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payer = Map::new();
        payer.insert("name".into(), json!(self.payer_name));
        payer.insert("phoneNumber".into(), json!(self.payer_phone_number));
        insert_opt(&mut payer, "email", self.payer_email.as_deref());
        insert_opt(&mut payer, "address", self.payer_address.as_deref());
        insert_opt(&mut payer, "bankId", self.payer_bank_id.as_deref());
        insert_opt(
            &mut payer,
            "bankAccountNumber",
            self.payer_bank_account_number.as_deref(),
        );
        insert_opt(&mut payer, "currency", self.payer_currency.as_deref());

        let mut payee = Map::new();
        payee.insert("name".into(), json!(self.payee_name));
        insert_opt(&mut payee, "accountNumber", self.payee_account_number.as_deref());
        insert_opt(&mut payee, "phoneNumber", self.payee_phone_number.as_deref());
        insert_opt(&mut payee, "bankId", self.payee_bank_id.as_deref());
        insert_opt(
            &mut payee,
            "bankAccountNumber",
            self.payee_bank_account_number.as_deref(),
        );

        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        payload.insert("payer".into(), Value::Object(payer));
        payload.insert("payee".into(), Value::Object(payee));
        payload.insert(
            "payerCollectionFeeShare".into(),
            json!(self.payer_collection_fee_share.unwrap_or(1.0)),
        );
        payload.insert(
            "payeeCollectionFeeShare".into(),
            json!(self.payee_collection_fee_share.unwrap_or(0.0)),
        );
        payload.insert(
            "paymentMethods".into(),
            json!(self
                .payment_methods
                .clone()
                .unwrap_or_else(|| vec!["BANK_TRANSFER".to_string()])),
        );
        insert_opt(&mut payload, "callBackUrl", self.call_back_url.as_deref());
        insert_opt(
            &mut payload,
            "expiryDateTimeUTC",
            self.expiry_date_time_utc.as_deref(),
        );
        if self.is_suppress_messages {
            payload.insert("isSuppressMessages".into(), json!(true));
        }
        if let Some(value) = self.is_single_use {
            payload.insert("isSingleUse".into(), json!(value));
        }
        if let Some(value) = self.is_allow_partial_payments {
            payload.insert("isAllowPartialPayments".into(), json!(value));
        }
        if let Some(value) = self.is_allow_over_payments {
            payload.insert("isAllowOverPayments".into(), json!(value));
        }
        insert_opt(&mut payload, "narration", self.narration.as_deref());
        insert_opt(&mut payload, "redirectUrl", self.redirect_url.as_deref());
        Value::Object(payload)
    }
}

/// Refund or cancel a payment request.
#[derive(Debug, Clone, Default)]
pub struct RefundPayment {
    pub reference_number: String,
    pub payment_request_reference: String,
    pub refund_amount: f64,
    pub currency: Option<String>,
    pub reason: Option<String>,
}

impl RefundPayment {
    pub fn new(
        reference_number: impl Into<String>,
        payment_request_reference: impl Into<String>,
        refund_amount: f64,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            payment_request_reference: payment_request_reference.into(),
            refund_amount,
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert(
            "paymentRequestReference".into(),
            json!(self.payment_request_reference),
        );
        payload.insert("refundAmount".into(), json!(self.refund_amount));
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        insert_opt(&mut payload, "reason", self.reason.as_deref());
        Value::Object(payload)
    }
}

/// Send money to a bank account or Paga wallet.
#[derive(Debug, Clone, Default)]
pub struct SendMoney {
    pub reference_number: String,
    /// Send as a string (e.g. `"5000.00"`): it is signed verbatim.
    pub amount: String,
    pub destination_bank_uuid: String,
    pub destination_bank_account_number: String,
    pub currency: Option<String>,
    pub remarks: Option<String>,
    pub status_callback_url: Option<String>,
    pub recipient_name: Option<String>,
    pub recipient_phone_number: Option<String>,
    pub recipient_mobile_operator_code: Option<String>,
    pub recipient_email: Option<String>,
    pub transfer_reference: Option<String>,
    pub suppress_recipient_message: Option<bool>,
    pub alternate_sender_name: Option<String>,
}

impl SendMoney {
    pub fn new(
        reference_number: impl Into<String>,
        amount: impl Into<String>,
        destination_bank_uuid: impl Into<String>,
        destination_bank_account_number: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount: amount.into(),
            destination_bank_uuid: destination_bank_uuid.into(),
            destination_bank_account_number: destination_bank_account_number.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        payload.insert("destinationBankUUID".into(), json!(self.destination_bank_uuid));
        payload.insert(
            "destinationBankAccountNumber".into(),
            json!(self.destination_bank_account_number),
        );
        insert_opt(&mut payload, "remarks", self.remarks.as_deref());
        insert_opt(
            &mut payload,
            "statusCallbackUrl",
            self.status_callback_url.as_deref(),
        );
        insert_opt(&mut payload, "recipientName", self.recipient_name.as_deref());
        insert_opt(
            &mut payload,
            "recipientPhoneNumber",
            self.recipient_phone_number.as_deref(),
        );
        insert_opt(
            &mut payload,
            "recipientMobileOperatorCode",
            self.recipient_mobile_operator_code.as_deref(),
        );
        insert_opt(&mut payload, "recipientEmail", self.recipient_email.as_deref());
        insert_opt(
            &mut payload,
            "transferReference",
            self.transfer_reference.as_deref(),
        );
        if let Some(value) = self.suppress_recipient_message {
            payload.insert("suppressRecipientMessage".into(), json!(value));
        }
        insert_opt(
            &mut payload,
            "alternateSenderName",
            self.alternate_sender_name.as_deref(),
        );
        Value::Object(payload)
    }
}

/// Buy airtime or data.
#[derive(Debug, Clone, Default)]
pub struct BuyAirtime {
    pub reference_number: String,
    pub amount: String,
    pub destination_phone_number: String,
    pub mobile_operator_public_id: String,
    pub currency: Option<String>,
    pub is_data_bundle: bool,
    pub mobile_operator_service_id: Option<String>,
    pub is_suppress_recipient_messages: bool,
}

impl BuyAirtime {
    pub fn new(
        reference_number: impl Into<String>,
        amount: impl Into<String>,
        destination_phone_number: impl Into<String>,
        mobile_operator_public_id: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount: amount.into(),
            destination_phone_number: destination_phone_number.into(),
            mobile_operator_public_id: mobile_operator_public_id.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert(
            "destinationPhoneNumber".into(),
            json!(self.destination_phone_number),
        );
        payload.insert(
            "mobileOperatorPublicId".into(),
            json!(self.mobile_operator_public_id),
        );
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        payload.insert("isDataBundle".into(), json!(self.is_data_bundle));
        insert_opt(
            &mut payload,
            "mobileOperatorServiceId",
            self.mobile_operator_service_id.as_deref(),
        );
        if self.is_suppress_recipient_messages {
            payload.insert("isSuppressRecipientMessages".into(), json!(true));
        }
        Value::Object(payload)
    }
}

/// Pay a bill from the business balance.
#[derive(Debug, Clone, Default)]
pub struct PayBill {
    pub reference_number: String,
    pub amount: f64,
    pub merchant_account: String,
    pub merchant_reference_number: String,
    pub merchant_service: Option<Vec<String>>,
    pub currency: Option<String>,
    pub payment_channel: Option<String>,
    pub locale: Option<String>,
    pub merchant_service_product_code: Option<String>,
}

impl PayBill {
    pub fn new(
        reference_number: impl Into<String>,
        amount: f64,
        merchant_account: impl Into<String>,
        merchant_reference_number: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount,
            merchant_account: merchant_account.into(),
            merchant_reference_number: merchant_reference_number.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert("merchantAccount".into(), json!(self.merchant_account));
        payload.insert(
            "merchantReferenceNumber".into(),
            json!(self.merchant_reference_number),
        );
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        if let Some(services) = &self.merchant_service {
            payload.insert("merchantService".into(), json!(services));
        }
        insert_opt(&mut payload, "paymentChannel", self.payment_channel.as_deref());
        insert_opt(&mut payload, "locale", self.locale.as_deref());
        insert_opt(
            &mut payload,
            "merchantServiceProductCode",
            self.merchant_service_product_code.as_deref(),
        );
        Value::Object(payload)
    }
}

/// The business transaction history.
#[derive(Debug, Clone, Default)]
pub struct GetBusinessTransactions {
    pub reference_number: String,
    pub start_date_utc: String,
    pub end_date_utc: String,
    pub locale: Option<String>,
}

impl GetBusinessTransactions {
    pub fn new(
        reference_number: impl Into<String>,
        start_date_utc: impl Into<String>,
        end_date_utc: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            start_date_utc: start_date_utc.into(),
            end_date_utc: end_date_utc.into(),
            locale: None,
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("startDateUTC".into(), json!(self.start_date_utc));
        payload.insert("endDateUTC".into(), json!(self.end_date_utc));
        insert_opt(&mut payload, "locale", self.locale.as_deref());
        Value::Object(payload)
    }
}

/// Create a subsidiary account.
#[derive(Debug, Clone, Default)]
pub struct CreateSubsidiaryAccount {
    pub reference_number: String,
    pub account_reference: String,
    pub account_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub iifi_number: Option<String>,
    pub funding_transaction_limit: Option<f64>,
    pub funding_daily_limit: Option<f64>,
    pub status: Option<String>,
    pub auto_sweep_destination: Option<String>,
    pub auto_sweep_bank_public_id: Option<String>,
    pub auto_sweep_account_number: Option<String>,
    pub callback_url: Option<String>,
}

impl CreateSubsidiaryAccount {
    pub fn new(
        reference_number: impl Into<String>,
        account_reference: impl Into<String>,
        account_name: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            account_reference: account_reference.into(),
            account_name: account_name.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("accountReference".into(), json!(self.account_reference));
        payload.insert("accountName".into(), json!(self.account_name));
        insert_opt(&mut payload, "firstName", self.first_name.as_deref());
        insert_opt(&mut payload, "lastName", self.last_name.as_deref());
        insert_opt(&mut payload, "phoneNumber", self.phone_number.as_deref());
        insert_opt(&mut payload, "email", self.email.as_deref());
        insert_opt(&mut payload, "iifiNumber", self.iifi_number.as_deref());
        if let Some(value) = self.funding_transaction_limit {
            payload.insert("fundingTransactionLimit".into(), json!(value));
        }
        if let Some(value) = self.funding_daily_limit {
            payload.insert("fundingDailyLimit".into(), json!(value));
        }
        insert_opt(&mut payload, "status", self.status.as_deref());
        if let Some(destination) = &self.auto_sweep_destination {
            let mut sweep = Map::new();
            sweep.insert("destination".into(), json!(destination));
            if destination == "BANK" {
                insert_opt(&mut sweep, "bankPublicId", self.auto_sweep_bank_public_id.as_deref());
                insert_opt(
                    &mut sweep,
                    "accountNumber",
                    self.auto_sweep_account_number.as_deref(),
                );
            }
            payload.insert("autoSweep".into(), Value::Object(sweep));
        }
        insert_opt(&mut payload, "callbackUrl", self.callback_url.as_deref());
        Value::Object(payload)
    }
}

/// Charge a ledger-backed subsidiary account.
#[derive(Debug, Clone, Default)]
pub struct ChargeSubsidiaryAccount {
    pub account_reference: String,
    pub reference_number: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub narration: Option<String>,
}

impl ChargeSubsidiaryAccount {
    pub fn new(
        account_reference: impl Into<String>,
        reference_number: impl Into<String>,
        amount: f64,
    ) -> Self {
        Self {
            account_reference: account_reference.into(),
            reference_number: reference_number.into(),
            amount,
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        money_payload(
            &self.reference_number,
            self.amount,
            self.currency.as_deref(),
            self.narration.as_deref(),
        )
    }
}

/// Top up a ledger-backed subsidiary account.
#[derive(Debug, Clone, Default)]
pub struct TopUpSubsidiaryAccount {
    pub account_reference: String,
    pub reference_number: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub narration: Option<String>,
}

impl TopUpSubsidiaryAccount {
    pub fn new(
        account_reference: impl Into<String>,
        reference_number: impl Into<String>,
        amount: f64,
    ) -> Self {
        Self {
            account_reference: account_reference.into(),
            reference_number: reference_number.into(),
            amount,
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        money_payload(
            &self.reference_number,
            self.amount,
            self.currency.as_deref(),
            self.narration.as_deref(),
        )
    }
}

fn money_payload(
    reference_number: &str,
    amount: f64,
    currency: Option<&str>,
    narration: Option<&str>,
) -> Value {
    let mut payload = Map::new();
    payload.insert("referenceNumber".into(), json!(reference_number));
    payload.insert("amount".into(), json!(amount));
    payload.insert(
        "currency".into(),
        json!(currency.unwrap_or("NGN")),
    );
    insert_opt(&mut payload, "narration", narration);
    Value::Object(payload)
}

/// Transfer money between two subsidiary accounts.
#[derive(Debug, Clone, Default)]
pub struct TransferBetweenSubsidiaryAccounts {
    pub source_account_reference: String,
    pub destination_account_reference: String,
    pub reference_number: String,
    pub amount: f64,
    pub currency: Option<String>,
    pub narration: Option<String>,
}

impl TransferBetweenSubsidiaryAccounts {
    pub fn new(
        source_account_reference: impl Into<String>,
        destination_account_reference: impl Into<String>,
        reference_number: impl Into<String>,
        amount: f64,
    ) -> Self {
        Self {
            source_account_reference: source_account_reference.into(),
            destination_account_reference: destination_account_reference.into(),
            reference_number: reference_number.into(),
            amount,
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert(
            "sourceAccountIdentifier".into(),
            json!(self.source_account_reference),
        );
        payload.insert(
            "destinationAccountIdentifier".into(),
            json!(self.destination_account_reference),
        );
        payload.insert("amount".into(), json!(self.amount));
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        insert_opt(&mut payload, "narration", self.narration.as_deref());
        Value::Object(payload)
    }
}

/// Update a subsidiary account.
#[derive(Debug, Clone, Default)]
pub struct UpdateSubsidiaryAccount {
    pub account_reference: String,
    pub reference_number: String,
    pub account_name: Option<String>,
    pub iifi_number: Option<String>,
    pub status: Option<String>,
    pub auto_sweep_destination: Option<String>,
    pub auto_sweep_bank_public_id: Option<String>,
    pub auto_sweep_account_number: Option<String>,
    pub callback_url: Option<String>,
}

impl UpdateSubsidiaryAccount {
    pub fn new(
        account_reference: impl Into<String>,
        reference_number: impl Into<String>,
    ) -> Self {
        Self {
            account_reference: account_reference.into(),
            reference_number: reference_number.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("accountIdentifier".into(), json!(self.account_reference));
        insert_opt(&mut payload, "accountName", self.account_name.as_deref());
        insert_opt(&mut payload, "iifiNumber", self.iifi_number.as_deref());
        insert_opt(&mut payload, "status", self.status.as_deref());
        if let Some(destination) = &self.auto_sweep_destination {
            let mut sweep = Map::new();
            sweep.insert("destination".into(), json!(destination));
            if destination == "BANK" {
                insert_opt(&mut sweep, "bankPublicId", self.auto_sweep_bank_public_id.as_deref());
                insert_opt(
                    &mut sweep,
                    "accountNumber",
                    self.auto_sweep_account_number.as_deref(),
                );
            }
            payload.insert("autoSweep".into(), Value::Object(sweep));
        }
        insert_opt(&mut payload, "callbackUrl", self.callback_url.as_deref());
        Value::Object(payload)
    }
}

/// A subsidiary account transaction history.
#[derive(Debug, Clone, Default)]
pub struct GetSubsidiaryHistory {
    pub account_reference: String,
    pub reference_number: String,
    pub start_date_time_utc: String,
    pub end_date_time_utc: String,
    pub page_number: Option<i64>,
    pub page_size: Option<i64>,
}

impl GetSubsidiaryHistory {
    pub fn new(
        account_reference: impl Into<String>,
        reference_number: impl Into<String>,
        start_date_time_utc: impl Into<String>,
        end_date_time_utc: impl Into<String>,
    ) -> Self {
        Self {
            account_reference: account_reference.into(),
            reference_number: reference_number.into(),
            start_date_time_utc: start_date_time_utc.into(),
            end_date_time_utc: end_date_time_utc.into(),
            ..Default::default()
        }
    }

    /// The payload the hash is computed over.
    pub fn to_payload(&self) -> Value {
        obj(vec![
            ("accountIdentifier", json!(self.account_reference)),
            ("startDateTimeUTC", json!(self.start_date_time_utc)),
            ("endDateTimeUTC", json!(self.end_date_time_utc)),
        ])
    }
}

/// Validate a destination bank account.
#[derive(Debug, Clone, Default)]
pub struct ValidateDepositToBank {
    pub reference_number: String,
    pub amount: String,
    pub destination_bank_uuid: String,
    pub destination_bank_account_number: String,
    pub currency: Option<String>,
    pub recipient_name: Option<String>,
    pub recipient_phone_number: Option<String>,
    pub recipient_mobile_operator_code: Option<String>,
    pub recipient_email: Option<String>,
    pub locale: Option<String>,
}

impl ValidateDepositToBank {
    pub fn new(
        reference_number: impl Into<String>,
        amount: impl Into<String>,
        destination_bank_uuid: impl Into<String>,
        destination_bank_account_number: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount: amount.into(),
            destination_bank_uuid: destination_bank_uuid.into(),
            destination_bank_account_number: destination_bank_account_number.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        payload.insert("destinationBankUUID".into(), json!(self.destination_bank_uuid));
        payload.insert(
            "destinationBankAccountNumber".into(),
            json!(self.destination_bank_account_number),
        );
        insert_opt(&mut payload, "recipientName", self.recipient_name.as_deref());
        insert_opt(
            &mut payload,
            "recipientPhoneNumber",
            self.recipient_phone_number.as_deref(),
        );
        insert_opt(
            &mut payload,
            "recipientMobileOperatorCode",
            self.recipient_mobile_operator_code.as_deref(),
        );
        insert_opt(&mut payload, "recipientEmail", self.recipient_email.as_deref());
        insert_opt(&mut payload, "locale", self.locale.as_deref());
        Value::Object(payload)
    }
}

/// Check a transaction's status.
#[derive(Debug, Clone, Default)]
pub struct GetTransactionStatus {
    pub reference_number: String,
    pub locale: Option<String>,
}

impl GetTransactionStatus {
    pub fn new(reference_number: impl Into<String>) -> Self {
        Self {
            reference_number: reference_number.into(),
            locale: None,
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        insert_opt(&mut payload, "locale", self.locale.as_deref());
        Value::Object(payload)
    }
}

/// The five most recent transactions on the business account.
#[derive(Debug, Clone, Default)]
pub struct GetRecentTransactions {
    pub reference_number: String,
    pub locale: Option<String>,
    pub account_principal: Option<String>,
    pub account_credentials: Option<String>,
}

impl GetRecentTransactions {
    pub fn new(reference_number: impl Into<String>) -> Self {
        Self {
            reference_number: reference_number.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        insert_opt(&mut payload, "locale", self.locale.as_deref());
        insert_opt(
            &mut payload,
            "accountPrincipal",
            self.account_principal.as_deref(),
        );
        insert_opt(
            &mut payload,
            "accountCredentials",
            self.account_credentials.as_deref(),
        );
        Value::Object(payload)
    }
}

/// Send money to a Paga wallet.
#[derive(Debug, Clone, Default)]
pub struct TransferToPagaWallet {
    pub reference_number: String,
    pub amount: f64,
    pub destination_account: String,
    pub currency: Option<String>,
    pub destination_bank: Option<String>,
    pub send_withdrawal_code: Option<bool>,
    pub source_of_funds: Option<String>,
    pub transfer_reference: Option<String>,
    pub suppress_recipient_message: Option<bool>,
    pub alternate_sender_name: Option<String>,
    pub locale: Option<String>,
}

impl TransferToPagaWallet {
    pub fn new(
        reference_number: impl Into<String>,
        amount: f64,
        destination_account: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount,
            destination_account: destination_account.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        payload.insert("destinationAccount".into(), json!(self.destination_account));
        if let Some(value) = self.send_withdrawal_code {
            payload.insert("sendWithdrawalCode".into(), json!(value));
        }
        if let Some(value) = self.suppress_recipient_message {
            payload.insert("suppressRecipientMessage".into(), json!(value));
        }
        insert_opt(&mut payload, "destinationBank", self.destination_bank.as_deref());
        insert_opt(&mut payload, "sourceOfFunds", self.source_of_funds.as_deref());
        insert_opt(
            &mut payload,
            "transferReference",
            self.transfer_reference.as_deref(),
        );
        insert_opt(
            &mut payload,
            "alternateSenderName",
            self.alternate_sender_name.as_deref(),
        );
        insert_opt(&mut payload, "locale", self.locale.as_deref());
        Value::Object(payload)
    }
}

/// Validate a Paga wallet transfer.
#[derive(Debug, Clone, Default)]
pub struct ValidateMoneyTransfer {
    pub reference_number: String,
    pub amount: f64,
    pub destination_account: String,
    pub currency: Option<String>,
}

impl ValidateMoneyTransfer {
    pub fn new(
        reference_number: impl Into<String>,
        amount: f64,
        destination_account: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount,
            destination_account: destination_account.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        payload.insert("destinationAccount".into(), json!(self.destination_account));
        Value::Object(payload)
    }
}

/// One item in a bulk money transfer.
#[derive(Debug, Clone, Default)]
pub struct BulkTransferItem {
    pub reference_number: String,
    pub amount: f64,
    pub destination_account: String,
    pub currency: Option<String>,
    pub destination_bank: Option<String>,
    pub transfer_reference: Option<String>,
    pub suppress_recipient_message: Option<bool>,
}

impl BulkTransferItem {
    pub fn new(
        reference_number: impl Into<String>,
        amount: f64,
        destination_account: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount,
            destination_account: destination_account.into(),
            ..Default::default()
        }
    }

    fn to_value(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert("destinationAccount".into(), json!(self.destination_account));
        insert_opt(&mut payload, "currency", self.currency.as_deref());
        insert_opt(&mut payload, "destinationBank", self.destination_bank.as_deref());
        insert_opt(
            &mut payload,
            "transferReference",
            self.transfer_reference.as_deref(),
        );
        if let Some(value) = self.suppress_recipient_message {
            payload.insert("suppressRecipientMessage".into(), json!(value));
        }
        Value::Object(payload)
    }
}

/// Send up to 300 money transfers in one call.
#[derive(Debug, Clone, Default)]
pub struct SendBulkMoneyTransfer {
    pub items: Vec<BulkTransferItem>,
    pub bulk_reference_number: Option<String>,
}

impl SendBulkMoneyTransfer {
    pub fn new(items: Vec<BulkTransferItem>) -> Self {
        Self {
            items,
            bulk_reference_number: None,
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert(
            "items".into(),
            Value::Array(self.items.iter().map(|item| item.to_value()).collect()),
        );
        insert_opt(
            &mut payload,
            "bulkReferenceNumber",
            self.bulk_reference_number.as_deref(),
        );
        Value::Object(payload)
    }

    /// The payload the hash is computed over: the first item plus the count.
    pub fn to_hash_payload(&self) -> Value {
        let first = &self.items[0];
        obj(vec![
            ("referenceNumber", json!(first.reference_number)),
            ("amount", json!(first.amount)),
            ("destinationAccount", json!(first.destination_account)),
            ("itemCount", json!(self.items.len())),
        ])
    }
}

/// Onboard a merchant.
#[derive(Debug, Clone, Default)]
pub struct OnboardMerchant {
    pub reference: String,
    pub merchant_external_id: String,
    pub legal_entity: Value,
    pub legal_entity_representative: Value,
    pub integration: Value,
    pub additional_parameters: Option<Value>,
}

impl OnboardMerchant {
    pub fn new(
        reference: impl Into<String>,
        merchant_external_id: impl Into<String>,
        legal_entity: Value,
        legal_entity_representative: Value,
        integration: Value,
    ) -> Self {
        Self {
            reference: reference.into(),
            merchant_external_id: merchant_external_id.into(),
            legal_entity,
            legal_entity_representative,
            integration,
            additional_parameters: None,
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut merchant_info = Map::new();
        merchant_info.insert("legalEntity".into(), self.legal_entity.clone());
        merchant_info.insert(
            "legalEntityRepresentative".into(),
            self.legal_entity_representative.clone(),
        );
        merchant_info.insert(
            "additionalParameters".into(),
            self.additional_parameters
                .clone()
                .unwrap_or_else(|| json!({})),
        );

        let mut payload = Map::new();
        payload.insert("reference".into(), json!(self.reference));
        payload.insert("merchantExternalId".into(), json!(self.merchant_external_id));
        payload.insert("merchantInfo".into(), Value::Object(merchant_info));
        payload.insert("integration".into(), self.integration.clone());
        Value::Object(payload)
    }

    /// The payload the hash is computed over.
    pub fn to_hash_payload(&self) -> Value {
        obj(vec![
            ("referenceNumber", json!(self.reference)),
            ("merchantExternalId", json!(self.merchant_external_id)),
            (
                "name",
                self.legal_entity.get("name").cloned().unwrap_or(Value::Null),
            ),
            (
                "phone",
                self.legal_entity_representative
                    .get("phone")
                    .cloned()
                    .unwrap_or(Value::Null),
            ),
            (
                "email",
                self.legal_entity_representative
                    .get("email")
                    .cloned()
                    .unwrap_or(Value::Null),
            ),
        ])
    }
}

/// Dispense cash against a customer's withdrawal code.
#[derive(Debug, Clone, Default)]
pub struct DispenseCash {
    pub reference_number: String,
    pub customer_phone_number: String,
    pub amount: f64,
    pub withdrawal_code: String,
    pub locale: Option<String>,
}

impl DispenseCash {
    pub fn new(
        reference_number: impl Into<String>,
        customer_phone_number: impl Into<String>,
        amount: f64,
        withdrawal_code: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            customer_phone_number: customer_phone_number.into(),
            amount,
            withdrawal_code: withdrawal_code.into(),
            locale: None,
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert(
            "customerPhoneNumber".into(),
            json!(self.customer_phone_number),
        );
        payload.insert("amount".into(), json!(self.amount));
        payload.insert("withdrawalCode".into(), json!(self.withdrawal_code));
        insert_opt(&mut payload, "locale", self.locale.as_deref());
        Value::Object(payload)
    }
}

/// Start a hosted Direct Debit set-up session.
#[derive(Debug, Clone, Default)]
pub struct StartDirectDebitSession {
    pub reference_number: String,
    pub account_reference: String,
    pub amount_limit: f64,
    pub callback_url: String,
    pub redirect_url: Option<String>,
    pub session_expiry_seconds: Option<String>,
    pub payment_expiry_seconds: Option<String>,
    pub payer_phone_number: Option<String>,
    pub payer_email: Option<String>,
    pub authorization_amount: Option<f64>,
}

impl StartDirectDebitSession {
    pub fn new(
        reference_number: impl Into<String>,
        account_reference: impl Into<String>,
        amount_limit: f64,
        callback_url: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            account_reference: account_reference.into(),
            amount_limit,
            callback_url: callback_url.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("accountReference".into(), json!(self.account_reference));
        payload.insert("amountLimit".into(), json!(self.amount_limit));
        payload.insert(
            "paymentMethods".into(),
            json!(vec!["BANK_ACCOUNT_DIRECT_DEBIT"]),
        );
        payload.insert("callbackUrl".into(), json!(self.callback_url));
        insert_opt(&mut payload, "redirectUrl", self.redirect_url.as_deref());
        insert_opt(
            &mut payload,
            "sessionExpirySeconds",
            self.session_expiry_seconds.as_deref(),
        );
        insert_opt(
            &mut payload,
            "paymentExpirySeconds",
            self.payment_expiry_seconds.as_deref(),
        );

        let mut additional = Map::new();
        insert_opt(
            &mut additional,
            "payerPhoneNumber",
            self.payer_phone_number.as_deref(),
        );
        insert_opt(&mut additional, "payerEmail", self.payer_email.as_deref());
        if let Some(value) = self.authorization_amount {
            additional.insert("authorizationAmount".into(), json!(value));
        }
        if !additional.is_empty() {
            payload.insert("additionalProperties".into(), Value::Object(additional));
        }
        Value::Object(payload)
    }
}

/// Set up a Direct Debit mandate by API.
#[derive(Debug, Clone, Default)]
pub struct CreateDirectDebitMandate {
    pub reference_number: String,
    pub amount: f64,
    pub payer_name: String,
    pub payer_phone_number: String,
    pub payee_name: String,
    pub account_reference: String,
    pub currency: Option<String>,
    pub payer_email: Option<String>,
    pub payer_address: Option<String>,
    pub payer_bank_id: Option<String>,
    pub payer_bank_account_number: Option<String>,
    pub payer_currency: Option<String>,
    pub call_back_url: Option<String>,
    pub redirect_url: Option<String>,
    pub narration: Option<String>,
    pub expiry_date_time_utc: Option<String>,
    pub is_single_use: Option<bool>,
    pub is_allow_partial_payments: Option<bool>,
    pub is_allow_over_payments: Option<bool>,
    pub is_suppress_messages: bool,
    pub payer_collection_fee_share: Option<f64>,
    pub payee_collection_fee_share: Option<f64>,
}

impl CreateDirectDebitMandate {
    pub fn new(
        reference_number: impl Into<String>,
        amount: f64,
        payer_name: impl Into<String>,
        payer_phone_number: impl Into<String>,
        payee_name: impl Into<String>,
        account_reference: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount,
            payer_name: payer_name.into(),
            payer_phone_number: payer_phone_number.into(),
            payee_name: payee_name.into(),
            account_reference: account_reference.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payer = Map::new();
        payer.insert("name".into(), json!(self.payer_name));
        payer.insert("phoneNumber".into(), json!(self.payer_phone_number));
        insert_opt(&mut payer, "email", self.payer_email.as_deref());
        insert_opt(&mut payer, "address", self.payer_address.as_deref());
        insert_opt(&mut payer, "bankId", self.payer_bank_id.as_deref());
        insert_opt(
            &mut payer,
            "bankAccountNumber",
            self.payer_bank_account_number.as_deref(),
        );
        insert_opt(&mut payer, "currency", self.payer_currency.as_deref());

        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("amount".into(), json!(self.amount));
        payload.insert(
            "currency".into(),
            json!(self.currency.clone().unwrap_or_else(|| "NGN".into())),
        );
        payload.insert("accountReference".into(), json!(self.account_reference));
        payload.insert("payer".into(), Value::Object(payer));
        payload.insert("payee".into(), json!({ "name": self.payee_name }));
        payload.insert("paymentMethods".into(), json!(vec!["DIRECT_DEBIT"]));
        payload.insert(
            "isSingleUse".into(),
            json!(self.is_single_use.unwrap_or(true)),
        );
        payload.insert(
            "isAllowPartialPayments".into(),
            json!(self.is_allow_partial_payments.unwrap_or(false)),
        );
        payload.insert(
            "isAllowOverPayments".into(),
            json!(self.is_allow_over_payments.unwrap_or(false)),
        );
        payload.insert(
            "payerCollectionFeeShare".into(),
            json!(self.payer_collection_fee_share.unwrap_or(1.0)),
        );
        payload.insert(
            "payeeCollectionFeeShare".into(),
            json!(self.payee_collection_fee_share.unwrap_or(0.0)),
        );
        insert_opt(&mut payload, "callBackUrl", self.call_back_url.as_deref());
        insert_opt(&mut payload, "redirectUrl", self.redirect_url.as_deref());
        insert_opt(&mut payload, "narration", self.narration.as_deref());
        insert_opt(
            &mut payload,
            "expiryDateTimeUTC",
            self.expiry_date_time_utc.as_deref(),
        );
        if self.is_suppress_messages {
            payload.insert("isSuppressMessages".into(), json!(true));
        }
        Value::Object(payload)
    }
}

/// Charge an approved Direct Debit mandate.
#[derive(Debug, Clone, Default)]
pub struct ChargeMandate {
    pub reference_number: String,
    pub amount: f64,
    pub account_reference: String,
}

impl ChargeMandate {
    pub fn new(
        reference_number: impl Into<String>,
        amount: f64,
        account_reference: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            amount,
            account_reference: account_reference.into(),
        }
    }

    pub fn to_payload(&self) -> Value {
        obj(vec![
            ("referenceNumber", json!(self.reference_number)),
            ("amount", json!(self.amount)),
            ("accountReference", json!(self.account_reference)),
        ])
    }
}

/// Disable a Direct Debit mandate.
#[derive(Debug, Clone, Default)]
pub struct DisableMandate {
    pub reference_number: String,
    pub account_reference: String,
}

impl DisableMandate {
    pub fn new(
        reference_number: impl Into<String>,
        account_reference: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            account_reference: account_reference.into(),
        }
    }

    pub fn to_payload(&self) -> Value {
        obj(vec![
            ("referenceNumber", json!(self.reference_number)),
            ("accountReference", json!(self.account_reference)),
        ])
    }
}

/// Register a persistent payment account.
#[derive(Debug, Clone, Default)]
pub struct RegisterPersistentPaymentAccount {
    pub reference_number: String,
    pub account_reference: String,
    pub financial_identification_number: String,
    pub credit_bank_id: String,
    pub credit_bank_account_number: String,
    pub callback_url: String,
}

impl RegisterPersistentPaymentAccount {
    pub fn new(
        reference_number: impl Into<String>,
        account_reference: impl Into<String>,
        financial_identification_number: impl Into<String>,
        credit_bank_id: impl Into<String>,
        credit_bank_account_number: impl Into<String>,
        callback_url: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            account_reference: account_reference.into(),
            financial_identification_number: financial_identification_number.into(),
            credit_bank_id: credit_bank_id.into(),
            credit_bank_account_number: credit_bank_account_number.into(),
            callback_url: callback_url.into(),
        }
    }

    pub fn to_payload(&self) -> Value {
        obj(vec![
            ("referenceNumber", json!(self.reference_number)),
            ("accountReference", json!(self.account_reference)),
            (
                "financialIdentificationNumber",
                json!(self.financial_identification_number),
            ),
            ("creditBankId", json!(self.credit_bank_id)),
            ("creditBankAccountNumber", json!(self.credit_bank_account_number)),
            ("callbackUrl", json!(self.callback_url)),
        ])
    }
}

/// Update a persistent payment account.
#[derive(Debug, Clone, Default)]
pub struct UpdatePersistentPaymentAccount {
    pub reference_number: String,
    pub account_identifier: String,
    pub financial_identification_number: Option<String>,
    pub credit_bank_id: Option<String>,
    pub credit_bank_account_number: Option<String>,
    pub callback_url: Option<String>,
}

impl UpdatePersistentPaymentAccount {
    pub fn new(
        reference_number: impl Into<String>,
        account_identifier: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            account_identifier: account_identifier.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert("accountIdentifier".into(), json!(self.account_identifier));
        insert_opt(
            &mut payload,
            "financialIdentificationNumber",
            self.financial_identification_number.as_deref(),
        );
        insert_opt(&mut payload, "creditBankId", self.credit_bank_id.as_deref());
        insert_opt(
            &mut payload,
            "creditBankAccountNumber",
            self.credit_bank_account_number.as_deref(),
        );
        insert_opt(&mut payload, "callbackUrl", self.callback_url.as_deref());
        Value::Object(payload)
    }
}

/// Register a customer.
#[derive(Debug, Clone, Default)]
pub struct RegisterCustomer {
    pub reference_number: String,
    pub customer_phone_number: String,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub customer_email: String,
    pub customer_date_of_birth: String,
    pub customer_address: Option<Value>,
    pub customer_reference: Option<String>,
}

impl RegisterCustomer {
    pub fn new(
        reference_number: impl Into<String>,
        customer_phone_number: impl Into<String>,
        customer_first_name: impl Into<String>,
        customer_last_name: impl Into<String>,
        customer_email: impl Into<String>,
        customer_date_of_birth: impl Into<String>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            customer_phone_number: customer_phone_number.into(),
            customer_first_name: customer_first_name.into(),
            customer_last_name: customer_last_name.into(),
            customer_email: customer_email.into(),
            customer_date_of_birth: customer_date_of_birth.into(),
            ..Default::default()
        }
    }

    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert(
            "customerPhoneNumber".into(),
            json!(self.customer_phone_number),
        );
        payload.insert(
            "customerFirstName".into(),
            json!(self.customer_first_name),
        );
        payload.insert("customerLastName".into(), json!(self.customer_last_name));
        payload.insert("customerEmail".into(), json!(self.customer_email));
        payload.insert(
            "customerDateOfBirth".into(),
            json!(self.customer_date_of_birth),
        );
        if let Some(address) = &self.customer_address {
            payload.insert("customerAddress".into(), address.clone());
        }
        insert_opt(
            &mut payload,
            "customerReference",
            self.customer_reference.as_deref(),
        );
        Value::Object(payload)
    }
}

/// Register a customer with KYC photographs.
#[derive(Debug, Clone, Default)]
pub struct RegisterCustomerWithKyc {
    pub reference_number: String,
    pub customer_phone_number: String,
    pub customer_email: String,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub customer_date_of_birth: String,
    pub customer_gender: String,
    /// The account photograph's bytes.
    pub customer_account_photo: Vec<u8>,
    /// The identification photograph's bytes.
    pub customer_id_photo: Vec<u8>,
    pub customer_address: Option<Value>,
    pub customer_marital_status: Option<String>,
    pub customer_preferred_language_iso_code: Option<String>,
    pub customer_id_type: Option<String>,
    pub customer_id_number: Option<String>,
    pub customer_id_expiration_date: Option<String>,
    pub optin_for_wallet_savings: Option<bool>,
    pub customer_reference: Option<String>,
}

impl RegisterCustomerWithKyc {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        reference_number: impl Into<String>,
        customer_phone_number: impl Into<String>,
        customer_email: impl Into<String>,
        customer_first_name: impl Into<String>,
        customer_last_name: impl Into<String>,
        customer_date_of_birth: impl Into<String>,
        customer_gender: impl Into<String>,
        customer_account_photo: Vec<u8>,
        customer_id_photo: Vec<u8>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            customer_phone_number: customer_phone_number.into(),
            customer_email: customer_email.into(),
            customer_first_name: customer_first_name.into(),
            customer_last_name: customer_last_name.into(),
            customer_date_of_birth: customer_date_of_birth.into(),
            customer_gender: customer_gender.into(),
            customer_account_photo,
            customer_id_photo,
            ..Default::default()
        }
    }

    /// The `customer` JSON part. The hash is computed over this.
    pub fn to_payload(&self) -> Value {
        let mut payload = Map::new();
        payload.insert("referenceNumber".into(), json!(self.reference_number));
        payload.insert(
            "customerPhoneNumber".into(),
            json!(self.customer_phone_number),
        );
        payload.insert("customerEmail".into(), json!(self.customer_email));
        payload.insert(
            "customerFirstName".into(),
            json!(self.customer_first_name),
        );
        payload.insert("customerLastName".into(), json!(self.customer_last_name));
        payload.insert(
            "customerDateOfBirth".into(),
            json!(self.customer_date_of_birth),
        );
        payload.insert("customerGender".into(), json!(self.customer_gender));
        if let Some(address) = &self.customer_address {
            payload.insert("customerAddress".into(), address.clone());
        }
        insert_opt(
            &mut payload,
            "customerMaritalStatus",
            self.customer_marital_status.as_deref(),
        );
        insert_opt(
            &mut payload,
            "customerPreferredLanguageISOCode",
            self.customer_preferred_language_iso_code.as_deref(),
        );
        insert_opt(
            &mut payload,
            "customerIdType",
            self.customer_id_type.as_deref(),
        );
        insert_opt(
            &mut payload,
            "customerIdNumber",
            self.customer_id_number.as_deref(),
        );
        insert_opt(
            &mut payload,
            "customerIdExpirationDate",
            self.customer_id_expiration_date.as_deref(),
        );
        if let Some(value) = self.optin_for_wallet_savings {
            payload.insert("optinForWalletSavings".into(), json!(value));
        }
        insert_opt(
            &mut payload,
            "customerReference",
            self.customer_reference.as_deref(),
        );
        Value::Object(payload)
    }
}

/// Upload a customer's account photograph.
#[derive(Debug, Clone, Default)]
pub struct RegisterCustomerAccountPhoto {
    pub reference_number: String,
    pub customer_phone_number: String,
    pub customer_account_photo: Vec<u8>,
}

impl RegisterCustomerAccountPhoto {
    pub fn new(
        reference_number: impl Into<String>,
        customer_phone_number: impl Into<String>,
        customer_account_photo: Vec<u8>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            customer_phone_number: customer_phone_number.into(),
            customer_account_photo,
        }
    }

    /// The `customer` JSON part. The hash is computed over this.
    pub fn to_payload(&self) -> Value {
        obj(vec![
            ("referenceNumber", json!(self.reference_number)),
            ("customerPhoneNumber", json!(self.customer_phone_number)),
        ])
    }
}

/// Upload a customer's identification.
#[derive(Debug, Clone, Default)]
pub struct RegisterCustomerIdentification {
    pub reference_number: String,
    pub customer_phone_number: String,
    pub customer_id_type: String,
    pub customer_id_number: String,
    pub customer_id_expiration_date: String,
    pub customer_id_photo: Vec<u8>,
}

impl RegisterCustomerIdentification {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        reference_number: impl Into<String>,
        customer_phone_number: impl Into<String>,
        customer_id_type: impl Into<String>,
        customer_id_number: impl Into<String>,
        customer_id_expiration_date: impl Into<String>,
        customer_id_photo: Vec<u8>,
    ) -> Self {
        Self {
            reference_number: reference_number.into(),
            customer_phone_number: customer_phone_number.into(),
            customer_id_type: customer_id_type.into(),
            customer_id_number: customer_id_number.into(),
            customer_id_expiration_date: customer_id_expiration_date.into(),
            customer_id_photo,
        }
    }

    /// The `customer` JSON part. The hash is computed over this.
    pub fn to_payload(&self) -> Value {
        obj(vec![
            ("referenceNumber", json!(self.reference_number)),
            ("customerPhoneNumber", json!(self.customer_phone_number)),
            ("customerIdType", json!(self.customer_id_type)),
            ("customerIdNumber", json!(self.customer_id_number)),
            (
                "customerIdExpirationDate",
                json!(self.customer_id_expiration_date),
            ),
        ])
    }
}
