use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::tag::Tag;

pub mod controller;
pub mod dto;
pub mod service;
pub mod vo;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Activity {
    pub id: u32,
    pub name: String,
    pub remark: String,
    pub color: String,
    pub icon: String,
    pub open: bool,
    pub tag_id: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActivityDetail {
    #[serde(flatten)]
    pub activity: Activity,
    pub tag: Tag,
}
