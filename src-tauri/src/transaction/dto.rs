use serde::Serialize;
use chrono::NaiveDateTime;
use sqlx::{sqlite::SqliteRow, Row};

use crate::tag::{service::get_tag_by_id, Tag};

use super::TransactionSplit;

#[derive(Serialize, Debug, Default)]
pub struct BalanceWithTypeDto {
    pub income: i32,
    pub expense: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransactionDto {
    pub id: u32,
    pub remark: String,
    pub wallet_name: String,
    pub to_wallet_name: Option<String>,
    pub currency: String,
    pub tag: Tag,
    pub amount: i32,
    pub time: NaiveDateTime,
    pub split: Option<TransactionSplit>,
}

impl TransactionDto {
    pub async fn try_from_row(row: &SqliteRow) -> crate::Result<Self> {
        Ok(TransactionDto {
            id: row.try_get("id")?,
            remark: row.try_get("remark")?,
            wallet_name: row.try_get("wallet_name")?,
            to_wallet_name: row.try_get("to_wallet_name").ok(),
            currency: row.try_get("currency")?,
            tag: get_tag_by_id(row.try_get("tag_id")?).await?,
            amount: row.try_get("amount")?,
            time: NaiveDateTime::parse_from_str(row.try_get("time")?, super::DATETIME_FORMAT)?,
            split: {
                let id: Option<u32> = row.try_get("split_id")?;
                if let Some(id) = id {
                    Some(super::service::get_split_by_id(id).await?)
                } else {
                    None
                }
            }
        })
    }
}