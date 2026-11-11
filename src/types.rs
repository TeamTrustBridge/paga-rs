/** Response and payload types for the Paga Engine API. */

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// The normalized envelope returned by Paga Engine API operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PagaResponse {
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub withdrawal_code: Option<String>,
    #[serde(default)]
    pub reversal_id: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub receiver_registration_status: Option<String>,
    #[serde(default)]
    pub recipient_account_holder_name: Option<String>,
    /// Any fields Paga adds that this SDK does not model yet.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectPaymentMethodResult {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub properties: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectPaymentRequestResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub request_amount: Option<f64>,
    #[serde(default)]
    pub total_payment_amount: Option<f64>,
    #[serde(default)]
    pub payment_methods: Option<Vec<CollectPaymentMethodResult>>,
    #[serde(default)]
    pub expiry_date_time_utc: Option<String>,
    #[serde(default)]
    pub is_payer_paga_account_holder: Option<bool>,
    #[serde(default)]
    pub bank_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutVerifyChargeResponse {
    #[serde(default, rename = "status_code")]
    pub status_code: Option<i64>,
    #[serde(default, rename = "status_message")]
    pub status_message: Option<String>,
    #[serde(default, rename = "chargeId")]
    pub charge_id: Option<String>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MerchantPaymentResponse {
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub response_category_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub merchant_transaction_reference: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub reversal_id: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub integration_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseResponse {
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub reversal_id: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub integration_status: Option<String>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalanceResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub total_balance: Option<String>,
    #[serde(default)]
    pub available_balance: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionItem {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub balance: Option<f64>,
    #[serde(default)]
    pub date_utc: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistoryResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub items: Option<Vec<TransactionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatusData {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub request_amount: Option<f64>,
    #[serde(default)]
    pub total_payment_amount: Option<f64>,
    #[serde(default)]
    pub expiry_date_time_utc: Option<String>,
    #[serde(default)]
    pub additional_data: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub refund_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentStatusResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub data: Option<PaymentStatusData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RefundStatusResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub data: Option<PaymentStatusData>,
}

impl RefundStatusResponse {
    /// The refund status of the underlying payment request.
    pub fn refund_status(&self) -> Option<&str> {
        self.data
            .as_ref()
            .and_then(|data| data.refund_status.as_deref())
    }

    /// Whether the payment request has been refunded.
    pub fn is_refunded(&self) -> bool {
        self.refund_status()
            .map(|status| status.trim().eq_ignore_ascii_case("refunded"))
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepositToBankResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub destination_account_holder_name_at_bank: Option<String>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub vat: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RefundResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub refund_amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub refund_destination: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PagaPaymentCallback {
    #[serde(default)]
    pub event: Option<String>,
    #[serde(default)]
    pub notification_id: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub external_reference_number: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub outstanding_balance: Option<f64>,
    #[serde(default)]
    pub payment_amount: Option<f64>,
    #[serde(default)]
    pub cumulative_payment_amount: Option<f64>,
    #[serde(default)]
    pub collection_fee: Option<f64>,
    #[serde(default)]
    pub funding_details: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryAccountResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub account_number: Option<String>,
    #[serde(default)]
    pub account_reference: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub balance: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryMoneyResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub new_balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryTransferResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub source: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub destination: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryBalanceResponse {
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub account_reference: Option<String>,
    #[serde(default)]
    pub account_number: Option<String>,
    #[serde(default)]
    pub balance: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub time_stamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubsidiaryHistoryResponse {
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub item_count: Option<i64>,
    #[serde(default)]
    pub total_items: Option<i64>,
    #[serde(default)]
    pub total_pages: Option<i64>,
    #[serde(default)]
    pub current_page: Option<i64>,
    #[serde(default)]
    pub transactions: Option<Vec<TransactionItem>>,
}

/* -------------------------------------------------------------------------- */
/* Directory lookups                                                           */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectBank {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectBanksResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub banks: Option<Vec<CollectBank>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Bank {
    #[serde(default)]
    pub uuid: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub inter_institution_code: Option<String>,
    #[serde(default)]
    pub sort_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BanksResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub banks: Option<Vec<Bank>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MobileOperator {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mobile_operator_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MobileOperatorsResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub mobile_operator: Option<Vec<MobileOperator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataBundle {
    #[serde(default)]
    pub service_id: Option<String>,
    #[serde(default)]
    pub service_name: Option<String>,
    #[serde(default)]
    pub service_price: Option<f64>,
    #[serde(default)]
    pub validity_period: Option<String>,
    #[serde(default)]
    pub data_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataBundlesResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub mobile_operator_services: Option<Vec<DataBundle>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Merchant {
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MerchantsResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub merchants: Option<Vec<Merchant>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MerchantService {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub short_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MerchantServicesResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub services: Option<Vec<MerchantService>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MerchantAccountDetailsResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub customer_name: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub account_number: Option<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub details: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDepositToBankResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub response_category_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub destination_account_holder_name_at_bank: Option<String>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub vat: Option<f64>,
}

/* -------------------------------------------------------------------------- */
/* Status lookups                                                              */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatusResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub response_category_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    /// Human-readable only. Branch on `response_code`, never on this.
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub transaction_reference: Option<String>,
    #[serde(default)]
    pub transaction_type: Option<String>,
    #[serde(default)]
    pub date_utc: Option<i64>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub merchant_transaction_reference: Option<String>,
    #[serde(default)]
    pub additional_properties: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub reversal_id: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub integration_status: Option<String>,
    #[serde(default)]
    pub commission_earned: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecentTransaction {
    #[serde(default)]
    pub item_number: Option<i64>,
    #[serde(default)]
    pub date_utc: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecentTransactionsResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub record_count: Option<i64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub items: Option<Vec<RecentTransaction>>,
}

/* -------------------------------------------------------------------------- */
/* Callback payloads                                                           */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoneyTransferNotification {
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub response_category_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub transaction_reference: Option<String>,
    #[serde(default)]
    pub transaction_type: Option<String>,
    #[serde(default)]
    pub transaction_date: Option<String>,
    #[serde(default)]
    pub amount: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub vat: Option<f64>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub destination_account_holder_name_at_bank: Option<String>,
    #[serde(default)]
    pub reversal_id: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub integration_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsdPaymentWebhook {
    #[serde(default)]
    pub event: Option<String>,
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub payment_id: Option<String>,
    #[serde(default)]
    pub payment_status: Option<String>,
    #[serde(default)]
    pub received_amounts: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub service_details: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub error_code: Option<String>,
    #[serde(default)]
    pub error_message: Option<HashMap<String, Value>>,
}

impl UsdPaymentWebhook {
    /// Whether the dollar payment cleared.
    pub fn is_captured(&self) -> bool {
        self.event
            .as_deref()
            .map(|event| event.trim().eq_ignore_ascii_case("CAPTURED"))
            .unwrap_or(false)
    }

    /// Whether the dollar payment failed to clear.
    pub fn is_funding_failed(&self) -> bool {
        self.event
            .as_deref()
            .map(|event| event.trim().eq_ignore_ascii_case("FUNDING_FAILED"))
            .unwrap_or(false)
    }
}

/* -------------------------------------------------------------------------- */
/* Direct Debit                                                                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectDebitSessionMetadata {
    #[serde(default)]
    pub amount_limit: Option<f64>,
    #[serde(default)]
    pub authorization_amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub account_reference: Option<String>,
    #[serde(default)]
    pub allow_partial_payment: Option<bool>,
    #[serde(default)]
    pub allow_multiple_use: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectDebitSessionData {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub session_key: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub checkout_url: Option<String>,
    #[serde(default)]
    pub redirect_url: Option<String>,
    #[serde(default)]
    pub callback_url: Option<String>,
    #[serde(default)]
    pub enabled_payment_methods: Option<Vec<String>>,
    #[serde(default)]
    pub initiated_at: Option<String>,
    #[serde(default)]
    pub session_expires_at: Option<String>,
    #[serde(default)]
    pub payment_expires_at: Option<String>,
    #[serde(default)]
    pub metadata: Option<DirectDebitSessionMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DirectDebitSessionResponse {
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub data: Option<DirectDebitSessionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MandateProcess {
    #[serde(default, rename = "ProcessStatusId")]
    pub process_status_id: Option<String>,
    #[serde(default, rename = "AccountReference")]
    pub account_reference: Option<String>,
    #[serde(default, rename = "StatusMessage")]
    pub status_message: Option<String>,
    #[serde(default, rename = "CreatedDateTimeUTC")]
    pub created_date_time_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MandateAdditionalData {
    #[serde(default)]
    pub mandate: Option<Vec<MandateProcess>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MandateStatusData {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub request_amount: Option<f64>,
    #[serde(default)]
    pub total_payment_amount: Option<f64>,
    #[serde(default)]
    pub payment_methods: Option<Vec<CollectPaymentMethodResult>>,
    #[serde(default)]
    pub expiry_date_time_utc: Option<String>,
    #[serde(default)]
    pub is_payer_paga_account_holder: Option<bool>,
    #[serde(default)]
    pub additional_data: Option<MandateAdditionalData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MandateStatusResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub data: Option<MandateStatusData>,
}

impl MandateStatusResponse {
    /// The mandate's `ProcessStatusId`, if Paga reported one.
    pub fn mandate_status(&self) -> Option<&str> {
        self.data
            .as_ref()
            .and_then(|data| data.additional_data.as_ref())
            .and_then(|additional| additional.mandate.as_ref())
            .and_then(|mandates| mandates.first())
            .and_then(|mandate| mandate.process_status_id.as_deref())
    }

    /// Whether the mandate may be charged.
    pub fn is_approved(&self) -> bool {
        self.mandate_status()
            .map(|status| status.trim().eq_ignore_ascii_case("APPROVED"))
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargeMandateResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChargeMandateStatusData {
    #[serde(default, rename = "statusCode")]
    pub status_code: Option<i64>,
    #[serde(default, rename = "ReferenceNumber")]
    pub reference_number: Option<String>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<f64>,
    #[serde(default, rename = "ProcessStatusId")]
    pub process_status_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargeMandateStatusResponse {
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub data: Option<ChargeMandateStatusData>,
}

impl ChargeMandateStatusResponse {
    /// The charge's own status code, when Paga reported one.
    pub fn charge_status(&self) -> Option<i64> {
        self.data.as_ref().and_then(|data| data.status_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisableMandateResponse {
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

/* -------------------------------------------------------------------------- */
/* Persistent accounts, customers and extra Business operations                */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersistentPaymentAccountResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub status_code: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub account_number: Option<String>,
    #[serde(default)]
    pub account_reference: Option<String>,
    #[serde(default)]
    pub financial_identification_number: Option<String>,
    #[serde(default)]
    pub credit_bank_id: Option<String>,
    #[serde(default)]
    pub credit_bank_account_number: Option<String>,
    #[serde(default)]
    pub callback_url: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub balance: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomerResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub response_category_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub customer_reference: Option<String>,
    #[serde(default)]
    pub customer_phone_number: Option<String>,
    #[serde(default)]
    pub customer_first_name: Option<String>,
    #[serde(default)]
    pub customer_last_name: Option<String>,
    #[serde(default)]
    pub customer_email: Option<String>,
    #[serde(default)]
    pub customer_date_of_birth: Option<String>,
    #[serde(default)]
    pub paga_account_number: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoneyTransferItem {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub destination_account: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub destination_bank: Option<String>,
    #[serde(default)]
    pub transfer_reference: Option<String>,
    #[serde(default)]
    pub suppress_recipient_message: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BulkTransferResult {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub withdrawal_code: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub receiver_registration_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BulkMoneyTransferResponse {
    #[serde(default)]
    pub bulk_reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub results: Option<Vec<BulkTransferResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MoneyTransferResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub response_category_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub withdrawal_code: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub reversal_id: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub receiver_registration_status: Option<String>,
    #[serde(default)]
    pub recipient_account_holder_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ValidateMoneyTransferResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub vat: Option<f64>,
    #[serde(default)]
    pub recipient_account_holder_name: Option<String>,
    #[serde(default)]
    pub recipient_registration_status: Option<String>,
    #[serde(default)]
    pub recipient_kyc_level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OperationStatusResponse {
    #[serde(default)]
    pub reference_number: Option<String>,
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub transaction_status: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub fee: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OnboardMerchantCredentials {
    #[serde(default)]
    pub merchant_public_id: Option<String>,
    #[serde(default)]
    pub merchant_secret_key: Option<String>,
    #[serde(default)]
    pub merchant_hmac: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OnboardMerchantUpdate {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub credentials: Option<OnboardMerchantCredentials>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OnboardMerchantResponse {
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub onboarding_update: Option<OnboardMerchantUpdate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DispenseCashResponse {
    #[serde(default)]
    pub response_code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub agent_commission: Option<f64>,
    #[serde(default)]
    pub customer_name: Option<String>,
}
