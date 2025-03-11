use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

pub mod controller;

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub id: u32,
    pub remark: String,
    pub wallet_id: u32,
    pub tag_id: u32,
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
            tag_id: row.try_get("tag_id")?,
            amount: row.try_get("amount")?,
            time: NaiveDateTime::parse_from_str(&time_str, DATETIME_FORMAT).map_err(|e| sqlx::Error::ColumnDecode { index: "time".to_string(), source: Box::new(e) })?,
        })
    }
}