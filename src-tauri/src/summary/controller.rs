use std::cmp::min;

use chrono::NaiveDate;
use tracing::{debug, info};
use sqlx::Row;

use crate::{sql::db, tag::TagKind, Result};

use super::{dto::{SimpleSummaryDto, SummaryDto, SummaryWithCurrencyDto}, SummaryType};

#[tauri::command]
pub async fn get_summary(summary_type: SummaryType, begin: Option<NaiveDate>, end: Option<NaiveDate>) -> Result<SummaryDto> {
    #[derive(Debug)]
    struct SingleQueryResult {
        date: String,
        income: i32,
        expense: i32,
    }

    #[derive(Debug)]
    struct QueryResult {
        currency: String,
        summary: Vec::<SingleQueryResult>,
    }

    debug!("Getting summary(type = {:?})", summary_type);
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    let data = sqlx::query(
        format!(r#"
        SELECT
            wallets.currency,
            SUM(
                CASE WHEN tags.kind = $4 THEN
                    CASE WHEN transactions.split_id IS NULL THEN transactions.amount ELSE ts.expense END
                ELSE 0 END
            ) AS expense,
            SUM(
                CASE WHEN tags.kind = $3 THEN transactions.amount ELSE 0 END
            ) AS income,
            {0} AS date
        FROM transactions
        JOIN wallets ON wallets.id = transactions.wallet_id
        JOIN tags ON tags.id = transactions.tag_id
        LEFT JOIN transaction_splits AS ts ON transactions.split_id = ts.id
        WHERE time BETWEEN $1 AND $2 AND tags.kind != $5
        GROUP BY wallets.currency, {0}
        ORDER BY wallets.currency, {0}
        "#, summary_type.to_sql("time")).as_str())
        .bind(begin.format(crate::transaction::DATETIME_FORMAT).to_string()).bind(end.format(crate::transaction::DATETIME_FORMAT).to_string())
        .bind(TagKind::Income as u8).bind(TagKind::Expense as u8).bind(TagKind::Transfer as u8)
        .fetch_all(db())
        .await?
        .iter()
        .fold(Vec::<QueryResult>::new(), |mut acc: Vec<QueryResult>, row| {
            let currency: String = row.get("currency");
            let date: String = row.get("date");
            let income: i32 = row.get("income");
            let expense: i32 = row.get("expense");

            if acc.last().map_or(false, |last| last.currency == currency) {
                acc.last_mut().unwrap().summary.push(SingleQueryResult { date, income, expense });
            } else {
                acc.push(QueryResult {
                    currency,
                    summary: vec![SingleQueryResult { date, income, expense }],
                });
            }
            acc
        });

    debug!("Generating dates for summary(type = {:?})", summary_type);
    let begin_date: String = sqlx::query(
        r#"
        SELECT MIN(time) AS date FROM transactions
        WHERE time BETWEEN $1 AND $2
        "#)
        .bind(begin.format(crate::transaction::DATETIME_FORMAT).to_string()).bind(end.format(crate::transaction::DATETIME_FORMAT).to_string())
        .fetch_one(db())
        .await?
        .get("date");
    let begin_date = NaiveDate::parse_from_str(&begin_date, crate::transaction::DATETIME_FORMAT).unwrap();
    let end_date = min(end, chrono::Local::now().naive_utc()).date();
    let mut dates = summary_type.generate_dates_between(begin_date, end_date);
    dates.reverse();
    info!("Dates(length = {}) generated", dates.len());

    let data = data.into_iter().map(|result| {
        let mut iter = result.summary.iter().rev();
        let mut current = iter.next();
        let summary = dates.iter().map(|date| {
            if current.as_ref().map_or(false, |c| c.date == *date) {
                let result = SimpleSummaryDto { income: current.unwrap().income, expense: current.unwrap().expense };
                current = iter.next();
                return result;
            } else {
                return SimpleSummaryDto { income: 0, expense: 0 };
            }
        }).collect::<Vec<SimpleSummaryDto>>();
        SummaryWithCurrencyDto {
            currency: result.currency,
            summary,
        }
    }).collect::<Vec<_>>();
    info!("Summary(type = {:?}) generated, currency count = {}, date count = {}", summary_type, data.len(), data.first().map_or(0, |d| d.summary.len()));
    Ok(SummaryDto { dates, data })
}