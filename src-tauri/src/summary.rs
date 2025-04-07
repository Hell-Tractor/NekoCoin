use serde::Deserialize;

pub mod controller;
mod dto;

#[derive(Deserialize, Debug, Clone)]
pub enum SummaryType {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl SummaryType {
    pub fn to_sql(&self, field_name: &str) -> String {
        match self {
            SummaryType::Daily => format!("strftime('%Y-%m-%d', {})", field_name),
            SummaryType::Weekly => format!("strftime('%Y-%W', {})", field_name),
            SummaryType::Monthly => format!("strftime('%Y-%m', {})", field_name),
            SummaryType::Yearly => format!("strftime('%Y', {})", field_name),
        }
    }
}