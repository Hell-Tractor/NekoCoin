use chrono::NaiveDate;
use tracing::{debug, info};
use sqlx::Row;

use crate::{sql::db, tag::TagKind, Result};

use super::{dto::{SimpleSummaryDto, SummaryWithCurrencyDto}, SummaryType};

#[tauri::command]
pub async fn get_summary(summary_type: SummaryType, begin: Option<NaiveDate>, end: Option<NaiveDate>) -> Result<Vec<SummaryWithCurrencyDto>> {
    debug!("Getting summary(type = {:?})", summary_type);
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    let result = sqlx::query(
        r#"
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
            $6 AS date
        FROM transactions
        JOIN wallets ON wallets.id = transactions.wallet_id
        JOIN tags ON tags.id = transactions.tag_id
        LEFT JOIN transaction_splits AS ts ON transactions.split_id = ts.id
        WHERE time BETWEEN $1 AND $2 AND tag.kind != $5
        GROUP BY wallets.currency, $6
        ORDER BY wallets.currency, $6
        "#)
        .bind(begin.format(crate::transaction::DATETIME_FORMAT).to_string()).bind(end.format(crate::transaction::DATETIME_FORMAT).to_string())
        .bind(TagKind::Income as u8).bind(TagKind::Expense as u8).bind(TagKind::Transfer as u8)
        .bind(summary_type.to_sql("time"))
        .fetch_all(db())
        .await?
        .iter()
        .fold(Vec::<SummaryWithCurrencyDto>::new(), |mut acc: Vec<SummaryWithCurrencyDto>, row| {
            let currency: String = row.get("currency");
            let date: String = row.get("date");
            let income: i32 = row.get("income");
            let expense: i32 = row.get("expense");

            if acc.last().map_or(false, |last| last.currency == currency) {
                acc.last_mut().unwrap().summary.push(SimpleSummaryDto { date, income, expense });
            } else {
                acc.push(SummaryWithCurrencyDto {
                    currency,
                    summary: vec![SimpleSummaryDto { date, income, expense }],
                });
            }
            acc
        });
    info!("Summary(type = {:?}) generated, currency count = {}, date count = {}", summary_type, result.len(), result.first().map_or(0, |d| d.summary.len()));
    Ok(result)
}