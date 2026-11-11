/** Response and payload types for the Paga Engine API. */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PagaResponse {
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub response_code: Option<i64>,
    pub message: Option<String>,
    pub reference_number: Option<String>,
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectPaymentMethodResult {
    pub name: Option<String>,
    pub properties: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectPaymentRequestResponse {
    pub reference_number: Option<String>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub request_amount: Option<f64>,
    pub total_payment_amount: Option<f64>,
    pub payment_methods: Option<Vec<CollectPaymentMethodResult>>,
    pub expiry_date_time_utc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutVerifyChargeResponse {
    pub status_code: Option<i64>,
    pub status_message: Option<String>,
    pub charge_id: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MerchantPaymentResponse {
    pub response_code: Option<i64>,
    pub response_category_code: Option<i64>,
    pub message: Option<String>,
    pub reference_number: Option<String>,
    pub merchant_transaction_reference: Option<String>,
    pub integration_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PurchaseResponse {
    pub response_code: Option<i64>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub transaction_id: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountBalanceResponse {
    pub reference_number: Option<String>,
    pub response_code: Option<i64>,
    pub total_balance: Option<String>,
    pub available_balance: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionItem {
    pub reference_number: Option<String>,
    pub status: Option<String>,
    pub amount: Option<f64>,
    pub fee: Option<f64>,
    pub balance: Option<f64>,
    pub date_utc: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionHistoryResponse {
    pub reference_number: Option<String>,
    pub response_code: Option<i64>,
    pub items: Option<Vec<TransactionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentStatusData {
    pub reference_number: Option<String>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub request_amount: Option<f64>,
    pub total_payment_amount: Option<f64>,
    pub expiry_date_time_utc: Option<String>,
    pub additional_data: Option<serde_json::Map<String, serde_json::Value>>,
    pub refund_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentStatusResponse {
    pub reference_number: Option<String>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub data: Option<Box<PaymentStatusData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DepositToBankResponse {
    pub reference_number: Option<String>,
    pub response_code: Option<i64>,
    pub message: Option<String>,
    pub transaction_id: Option<String>,
    pub destination_account_holder_name_at_bank: Option<String>,
    pub fee: Option<f64>,
    pub vat: Option<f64>,
    pub currency: Option<String>,
    pub exchange_rate: Option<f64>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundResponse {
    pub reference_number: Option<String>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub refund_amount: Option<f64>,
    pub currency: Option<String>,
    pub refund_destination: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PagaPaymentCallback {
    pub event: Option<String>,
    pub notification_id: Option<String>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub external_reference_number: Option<String>,
    pub state: Option<String>,
    pub outstanding_balance: Option<f64>,
    pub payment_amount: Option<f64>,
    pub cumulative_payment_amount: Option<f64>,
    pub collection_fee: Option<f64>,
    pub funding_details: Option<serde_json::Map<String, serde_json::Value>>,
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubsidiaryAccountResponse {
    pub reference_number: Option<String>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub account_number: Option<String>,
    pub account_reference: Option<String>,
    pub status: Option<String>,
    pub balance: Option<f64>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubsidiaryMoneyResponse {
    pub reference_number: Option<String>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub transaction_id: Option<String>,
    pub new_balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubsidiaryTransferResponse {
    pub reference_number: Option<String>,
    pub status_code: Option<String>,
    pub status_message: Option<String>,
    pub transaction_id: Option<String>,
    pub source: Option<serde_json::Map<String, serde_json::Value>>,
    pub destination: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubsidiaryBalanceResponse {
    pub status_code: Option<String>,
    pub account_reference: Option<String>,
    pub account_number: Option<String>,
    pub balance: Option<f64>,
    pub currency: Option<String>,
    pub time_stamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubsidiaryHistoryResponse {
    pub status_code: Option<String>,
    pub item_count: Option<i64>,
    pub total_items: Option<i64>,
    pub total_pages: Option<i64>,
    pub current_page: Option<i64>,
    pub transactions: Option<Vec<TransactionItem>>,
}
