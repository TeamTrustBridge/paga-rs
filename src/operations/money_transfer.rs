//! Money transfer operations for the Paga Engine API.
//!
//! Covers the **Make payments → Money Transfer** section of Paga's
//! documentation.

#[allow(unused_imports)]
use crate::auth::*;
use crate::client::{PagaClient, RequestOptions};
use crate::errors::PagaError;
#[allow(unused_imports)]
use crate::requests::*;
#[allow(unused_imports)]
use crate::types::*;
use reqwest::Method;

impl PagaClient {
    /// List every institution you can pay.
    pub async fn get_banks(&self, reference_number: &str) -> Result<BanksResponse, PagaError> {
        let payload = serde_json::json!({ "referenceNumber": reference_number });
        self.request(
            Method::POST,
            "/getBanks",
            Some(&payload),
            RequestOptions::secured(BANKS_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Validate a destination bank account.
    pub async fn validate_deposit_to_bank(
        &self,
        request: ValidateDepositToBank,
    ) -> Result<ValidateDepositToBankResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/validateDepositToBank",
            Some(&payload),
            RequestOptions::secured(VALIDATE_DEPOSIT_TO_BANK_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Send money to a bank account or Paga wallet.
    pub async fn send_money(&self, request: SendMoney) -> Result<DepositToBankResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/depositToBank",
            Some(&payload),
            RequestOptions::secured(MONEY_TRANSFER_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Send money to a Paga wallet.
    pub async fn transfer_to_paga_wallet(
        &self,
        request: TransferToPagaWallet,
    ) -> Result<MoneyTransferResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/moneyTransfer",
            Some(&payload),
            RequestOptions::secured(PAGA_WALLET_TRANSFER_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Validate a Paga wallet transfer.
    pub async fn validate_money_transfer(
        &self,
        request: ValidateMoneyTransfer,
    ) -> Result<ValidateMoneyTransferResponse, PagaError> {
        let payload = request.to_payload();
        self.request(
            Method::POST,
            "/validateMoneyTransfer",
            Some(&payload),
            RequestOptions::secured(VALIDATE_MONEY_TRANSFER_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .lenient(),
        )
        .await
    }

    /// Send up to 300 money transfers in one call.
    pub async fn send_bulk_money_transfer(
        &self,
        request: SendBulkMoneyTransfer,
    ) -> Result<BulkMoneyTransferResponse, PagaError> {
        if request.items.is_empty() {
            return Err(PagaError::new("items must not be empty"));
        }
        if request.items.len() > 300 {
            return Err(PagaError::new(
                "a bulk money transfer may contain at most 300 items",
            ));
        }
        let payload = request.to_payload();
        let hash_payload = request.to_hash_payload();
        self.request(
            Method::POST,
            "/moneyTransferBulk",
            Some(&payload),
            RequestOptions::secured(MONEY_TRANSFER_BULK_HASH_FIELDS)
                .base_url(self.business_base_url.clone())
                .hash_payload(hash_payload)
                .lenient(),
        )
        .await
    }
}
