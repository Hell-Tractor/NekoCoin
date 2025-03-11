use sqlx::FromRow;

use crate::money::Money;

pub mod controller;

#[derive(Debug, Clone, FromRow)]
struct Wallet {
    pub id: u32,
    pub name: String,
    pub remark: String,
    pub balance: Money,
}
