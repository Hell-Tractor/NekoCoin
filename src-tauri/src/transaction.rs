use chrono::NaiveDateTime;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

use crate::{tag::{self, Tag}, wallet::{self, Wallet}};

pub mod controller;
pub mod dto;
pub mod vo;
pub mod service;

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Transaction {
    pub id: u32,
    pub remark: String,
    pub wallet_id: u32,
    wallet: Option<Wallet>,
    pub to_wallet_id: Option<u32>,
    to_wallet: Option<Wallet>,
    pub tag_id: u32,
    tag: Option<Tag>,
    pub amount: i32,
    pub time: NaiveDateTime,
}

impl<'r> FromRow<'r, SqliteRow> for Transaction {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let time_str: String = row.try_get("time")?;
        Ok(Transaction {
            id: row.try_get("id")?,
            remark: row.try_get("remark")?,
            wallet_id: row.try_get("wallet_id")?,
            to_wallet_id: row.try_get("to_wallet_id")?,
            tag_id: row.try_get("tag_id")?,
            amount: row.try_get("amount")?,
            time: NaiveDateTime::parse_from_str(&time_str, DATETIME_FORMAT).map_err(|e| sqlx::Error::ColumnDecode { index: "time".to_string(), source: Box::new(e) })?,
            wallet: None,
            to_wallet: None,
            tag: None,
        })
    }
}

#[allow(dead_code)]
impl Transaction {
    /*
     * get_wallet, get_to_wallet, get_tag occupied
     * mutable borrow of self with same lifetime of returned reference
     * this should be updated to some other methods like OnceCell in the future
    */
    pub async fn get_wallet(&mut self) -> crate::Result<&Wallet> {
        if self.wallet.is_none() || self.wallet.as_ref().unwrap().id != self.wallet_id {
            self.wallet = Some(wallet::service::get_wallet_by_id(self.wallet_id).await?);
        }
        Ok(self.wallet.as_ref().unwrap())
    }

    pub async fn get_to_wallet(&mut self) -> crate::Result<Option<&Wallet>> {
        if self.to_wallet_id.is_none() {
            return Ok(None);
        }
        if self.to_wallet.is_none() || self.to_wallet.as_ref().unwrap().id != self.to_wallet_id.unwrap() {
            self.to_wallet = Some(wallet::service::get_wallet_by_id(self.to_wallet_id.unwrap()).await?);
        }
        Ok(self.to_wallet.as_ref())
    }

    pub async fn get_tag(&mut self) -> crate::Result<&Tag> {
        if self.tag.is_none() || self.tag.as_ref().unwrap().id != self.tag_id {
            self.tag = Some(tag::service::get_tag_by_id(self.tag_id).await?);
        }
        Ok(self.tag.as_ref().unwrap())
    }
}