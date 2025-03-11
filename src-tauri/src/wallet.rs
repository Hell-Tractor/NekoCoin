use serde::Serialize;
use sqlx::FromRow;

use crate::money::Money;

pub mod controller;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Wallet {
    pub id: u32,
    pub name: String,
    pub remark: String,
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub balance: Money,
    pub color: String,
    pub icon: String,
}
