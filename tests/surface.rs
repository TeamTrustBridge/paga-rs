#[test]
fn every_operation_is_reachable() {
    use paga_rs::PagaClient;
    let client = PagaClient::new("a", "b", "c", "test").unwrap();
    let required: Vec<(&str, fn(&PagaClient))> = vec![
        ("create_payment", |c| { let _ = c.create_payment(paga_rs::CreatePayment::new("r", 1.0, "n", "p", "y")); }),
        ("get_payment_status", |c| { let _ = c.get_payment_status("r"); }),
        ("verify_charge", |c| { let _ = c.verify_charge("r", "k", 1.0); }),
        ("refund_payment", |c| { let _ = c.refund_payment(paga_rs::RefundPayment::new("r", "p", 1.0)); }),
        ("get_refund_status", |c| { let _ = c.get_refund_status("r"); }),
        ("get_collect_banks", |c| { let _ = c.get_collect_banks("r"); }),
        ("get_usd_payment_status", |c| { let _ = c.get_usd_payment_status("r", "USD"); }),
        ("get_payment_request_history", |c| { let _ = c.get_payment_request_history("r"); }),
        ("get_banks", |c| { let _ = c.get_banks("r"); }),
        ("validate_deposit_to_bank", |c| { let _ = c.validate_deposit_to_bank(paga_rs::ValidateDepositToBank::new("r", "1", "u", "a")); }),
        ("send_money", |c| { let _ = c.send_money(paga_rs::SendMoney::new("r", "1", "u", "a")); }),
        ("transfer_to_paga_wallet", |c| { let _ = c.transfer_to_paga_wallet(paga_rs::TransferToPagaWallet::new("r", 1.0, "d")); }),
        ("validate_money_transfer", |c| { let _ = c.validate_money_transfer(paga_rs::ValidateMoneyTransfer::new("r", 1.0, "d")); }),
        ("send_bulk_money_transfer", |c| { let _ = c.send_bulk_money_transfer(paga_rs::SendBulkMoneyTransfer::new(vec![])); }),
        ("get_transaction_status", |c| { let _ = c.get_transaction_status(paga_rs::GetTransactionStatus::new("r")); }),
        ("get_mobile_operators", |c| { let _ = c.get_mobile_operators("r", None); }),
        ("get_data_bundles", |c| { let _ = c.get_data_bundles("r", "o"); }),
        ("buy_airtime", |c| { let _ = c.buy_airtime(paga_rs::BuyAirtime::new("r", "1", "p", "o")); }),
        ("get_merchants", |c| { let _ = c.get_merchants("r"); }),
        ("get_merchant_services", |c| { let _ = c.get_merchant_services("r", "m"); }),
        ("get_merchant_account_details", |c| { let _ = c.get_merchant_account_details("r", "m", "n", None); }),
        ("pay_bill", |c| { let _ = c.pay_bill(paga_rs::PayBill::new("r", 1.0, "m", "n")); }),
        ("get_business_balance", |c| { let _ = c.get_business_balance("r"); }),
        ("get_business_transactions", |c| { let _ = c.get_business_transactions(paga_rs::GetBusinessTransactions::new("r", "s", "e")); }),
        ("get_recent_transactions", |c| { let _ = c.get_recent_transactions(paga_rs::GetRecentTransactions::new("r")); }),
        ("get_operation_status", |c| { let _ = c.get_operation_status(paga_rs::GetTransactionStatus::new("r")); }),
        ("onboard_merchant", |c| { let _ = c.onboard_merchant(paga_rs::OnboardMerchant::new("r", "m", serde_json::json!({}), serde_json::json!({}), serde_json::json!({}))); }),
        ("dispense_cash", |c| { let _ = c.dispense_cash(paga_rs::DispenseCash::new("r", "p", 1.0, "w")); }),
        ("create_subsidiary_account", |c| { let _ = c.create_subsidiary_account(paga_rs::CreateSubsidiaryAccount::new("r", "a", "n")); }),
        ("charge_subsidiary_account", |c| { let _ = c.charge_subsidiary_account(paga_rs::ChargeSubsidiaryAccount::new("a", "r", 1.0)); }),
        ("top_up_subsidiary_account", |c| { let _ = c.top_up_subsidiary_account(paga_rs::TopUpSubsidiaryAccount::new("a", "r", 1.0)); }),
        ("transfer_between_subsidiary_accounts", |c| { let _ = c.transfer_between_subsidiary_accounts(paga_rs::TransferBetweenSubsidiaryAccounts::new("s", "d", "r", 1.0)); }),
        ("retrieve_subsidiary_account", |c| { let _ = c.retrieve_subsidiary_account("a", "r"); }),
        ("update_subsidiary_account", |c| { let _ = c.update_subsidiary_account(paga_rs::UpdateSubsidiaryAccount::new("a", "r")); }),
        ("get_subsidiary_balance", |c| { let _ = c.get_subsidiary_balance("a", "r"); }),
        ("get_subsidiary_history", |c| { let _ = c.get_subsidiary_history(paga_rs::GetSubsidiaryHistory::new("a", "r", "s", "e")); }),
        ("register_persistent_payment_account", |c| { let _ = c.register_persistent_payment_account(paga_rs::RegisterPersistentPaymentAccount::new("r", "a", "f", "b", "n", "u")); }),
        ("delete_persistent_payment_account", |c| { let _ = c.delete_persistent_payment_account("r", "a"); }),
        ("update_persistent_payment_account", |c| { let _ = c.update_persistent_payment_account(paga_rs::UpdatePersistentPaymentAccount::new("r", "a")); }),
        ("get_persistent_payment_account", |c| { let _ = c.get_persistent_payment_account("r", "a"); }),
        ("register_customer", |c| { let _ = c.register_customer(paga_rs::RegisterCustomer::new("r", "p", "f", "l", "e", "d")); }),
        ("register_customer_with_kyc", |c| { let _ = c.register_customer_with_kyc(paga_rs::RegisterCustomerWithKyc::new("r", "p", "e", "f", "l", "d", "g", vec![], vec![])); }),
        ("register_customer_account_photo", |c| { let _ = c.register_customer_account_photo(paga_rs::RegisterCustomerAccountPhoto::new("r", "p", vec![])); }),
        ("register_customer_identification", |c| { let _ = c.register_customer_identification(paga_rs::RegisterCustomerIdentification::new("r", "p", "t", "n", "e", vec![])); }),
        ("start_direct_debit_session", |c| { let _ = c.start_direct_debit_session(paga_rs::StartDirectDebitSession::new("r", "a", 1.0, "u")); }),
        ("create_direct_debit_mandate", |c| { let _ = c.create_direct_debit_mandate(paga_rs::CreateDirectDebitMandate::new("r", 1.0, "n", "p", "y", "a")); }),
        ("get_mandate_status", |c| { let _ = c.get_mandate_status("r"); }),
        ("charge_mandate", |c| { let _ = c.charge_mandate(paga_rs::ChargeMandate::new("r", 1.0, "a")); }),
        ("get_charge_mandate_status", |c| { let _ = c.get_charge_mandate_status("r"); }),
        ("disable_mandate", |c| { let _ = c.disable_mandate(paga_rs::DisableMandate::new("r", "a")); }),
    ];
    // The closures above name every operation, so this file failing to
    // compile is the actual test: an operation that was lost or renamed
    // would break the build.
    assert!(!required.is_empty());
    let _ = client;
}
