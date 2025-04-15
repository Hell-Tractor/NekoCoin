use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SummaryWithCurrencyDto {
    pub currency: String,
    pub summary: Vec<SimpleSummaryDto>,
}

#[derive(Serialize, Debug)]
pub struct SimpleSummaryDto {
    // pub date: String,
    pub income: i32,
    pub expense: i32,
}

#[derive(Serialize, Debug)]
pub struct SummaryDto {
    pub dates: Vec<String>,
    pub data: Vec<SummaryWithCurrencyDto>,
}