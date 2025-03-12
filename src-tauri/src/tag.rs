use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod controller;
pub mod service;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[repr(u8)]
pub enum TagKind {
    Expense,
    Income,
    Transfer,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub remark: String,
    pub color: String,
    pub icon: String,
    #[serde(rename = "type")]
    pub kind: TagKind,
    pub parent_id: Option<u32>,
}