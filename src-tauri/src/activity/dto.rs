use serde::Serialize;

use crate::tag::Tag;

#[derive(Debug, Clone, Serialize)]
pub struct ActivityBriefDto {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub open: bool,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SummaryByActivityDto {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub open: bool,
    pub summary: i32,
    pub currency_code: String,
    pub period: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ActivityBalanceDto {
    pub id: u32,
    pub income: i32,
    pub expense: i32,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SummaryByActivityTagDto {
    #[sqlx(flatten)]
    pub tag: Tag,
    pub summary: i32,
    pub currency_code: String,
    pub period: String,
}
