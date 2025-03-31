use chrono::NaiveDateTime;
use serde::Deserialize;

use super::{Transaction, DATETIME_FORMAT};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionVo {
    pub id: u32,
    pub remark: String,
    pub wallet_id: u32,
    #[serde(default)]
    pub to_wallet_id: Option<u32>,
    pub tag_id: u32,
    pub amount: i32,
    pub time: String,
    pub split: Option<TransactionSplitVo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSplitVo {
    pub id: Option<u32>,
    pub count: u32,
    pub expense: i32,
    pub recieve_wallet_id: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionSplitVo {
    pub count: u32,
    pub expense: i32,
    pub recieve_wallet_id: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionVo {
    pub remark: String,
    pub wallet_id: u32,
    pub to_wallet_id: Option<u32>,
    pub tag_id: u32,
    pub amount: i32,
    pub time: String,
    pub split: Option<CreateTransactionSplitVo>,
}

impl Into<Transaction> for TransactionVo {
    fn into(self) -> Transaction {
        Transaction {
            id: self.id,
            remark: self.remark,
            wallet_id: self.wallet_id,
            to_wallet_id: self.to_wallet_id,
            tag_id: self.tag_id,
            split_id: self.split.iter().flat_map(|s| s.id).next(),
            amount: self.amount,
            time: NaiveDateTime::parse_from_str(&self.time, DATETIME_FORMAT).unwrap(),
            wallet: None,
            to_wallet: None,
            tag: None,
            split: None,
        }
    }
}