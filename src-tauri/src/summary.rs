use chrono::{Datelike, Days, Months, NaiveDate};
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
    pub fn format_datetime(&self, datetime: NaiveDate) -> String {
        match self {
            SummaryType::Daily => datetime.format("%Y-%m-%d").to_string(),
            SummaryType::Weekly => datetime.format("%Y-%W").to_string(),
            SummaryType::Monthly => datetime.format("%Y-%m").to_string(),
            SummaryType::Yearly => datetime.format("%Y").to_string(),
        }
    }
    pub fn generate_date_values_between(&self, begin: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
        let mut dates = vec![];
        let mut current_date = begin;
        while self.get_range_of_date(current_date).0 <= end {
            dates.push(current_date);
            current_date = match self {
                SummaryType::Daily => current_date.succ_opt().unwrap(),
                SummaryType::Weekly => current_date + chrono::Duration::weeks(1),
                SummaryType::Monthly => current_date.checked_add_months(Months::new(1)).unwrap(),
                SummaryType::Yearly => current_date.with_year(current_date.year() + 1).unwrap(),
            };
        }
        dates
    }
    pub fn get_range_of_date(&self, date: NaiveDate) -> (NaiveDate, NaiveDate) {
        match self {
            SummaryType::Daily => (date, date),
            SummaryType::Weekly => {
                let week_begin = date.checked_sub_days(Days::new(date.weekday().num_days_from_monday().into())).unwrap();
                (week_begin, week_begin.checked_add_days(Days::new(6)).unwrap())
            },
            SummaryType::Monthly => (date.with_day(1).unwrap(), date.checked_add_months(Months::new(1)).unwrap().with_day(1).unwrap().checked_sub_days(Days::new(1)).unwrap()),
            SummaryType::Yearly => (date.with_month(1).unwrap().with_day(1).unwrap(), date.with_month(12).unwrap().with_day(31).unwrap()),
        }
    }
}