use chrono::{NaiveDate, NaiveDateTime};
use tracing::{debug, info};

use crate::sql::db;
use crate::{tag, wallet, Error, Result};

use super::Transaction;

#[tauri::command]
pub async fn create_transaction(remark: String, wallet_id: u32, tag_id: u32, amount: i32, time: String) -> Result<()> {
    debug!("Creating transaction: {} {} {} {} {}", remark, wallet_id, tag_id, amount, time);
    let time = NaiveDateTime::parse_from_str(&time, super::DATETIME_FORMAT)?;
    let mut tx = db().begin().await?;

    let tag = tag::service::get_tag_by_id(tag_id).await?;
    if tag.kind != tag::TagKind::Expense && tag.kind != tag::TagKind::Income {
        return Err(Error::InvalidTagType {
            given: tag.kind,
            allow: vec![tag::TagKind::Expense, tag::TagKind::Income]
        });
    }
    debug!("Tag kind is valid, modifying wallet amount and recording transaction simultaneously...");

    if tag.kind == tag::TagKind::Expense {
        wallet::service::modify_currency(wallet_id, -amount).await?;
    } else {  // tag.kind == tag::TagKind::Income
        wallet::service::modify_currency(wallet_id, amount).await?;
    };

    sqlx::query(
        r#"
        INSERT INTO transactions (remark, wallet_id, tag_id, amount, time)
        VALUES ($1, $2, $3, $4, $5)
        "#)
        .bind(remark).bind(wallet_id).bind(tag_id).bind(amount).bind(time.format(super::DATETIME_FORMAT).to_string())
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    info!("Transaction created");
    Ok(())
}

#[tauri::command]
pub async fn retrieve_transactions(begin: Option<NaiveDate>, end: Option<NaiveDate>, page: u32, page_size: u32) -> Result<Vec<Transaction>> {
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap());
    let transactions = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, remark, wallet_id, tag_id, amount, time
        FROM transactions
        WHERE date between $1 and $2
        ORDER BY date DESC
        LIMIT $3 OFFSET $4
        "#)
        .bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .bind(page_size).bind(page * page_size)
        .fetch_all(db())
        .await?;
    info!("Retrieved {} transactions.", transactions.len());
    Ok(transactions)
}

#[tauri::command]
pub async fn retrieve_transactions_in_wallet(wallet_id: u32, begin: Option<NaiveDate>, end: Option<NaiveDate>, page: u32, page_size: u32) -> Result<Vec<Transaction>> {
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap());
    let transactions = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, remark, wallet_id, tag_id, amount, time
        FROM transactions
        WHERE wallet_id = $1 AND date between $2 and $3
        ORDER BY date DESC
        LIMIT $4 OFFSET $5
        "#)
        .bind(wallet_id).bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .bind(page_size).bind(page * page_size)
        .fetch_all(db())
        .await?;
    info!("Retrieved {} transactions in wallet `{}`.", transactions.len(), wallet_id);
    Ok(transactions)
}

#[tauri::command]
pub async fn retrieve_transactions_with_tag(tag_id: u32, begin: Option<NaiveDate>, end: Option<NaiveDate>, page: u32, page_size: u32) -> Result<Vec<Transaction>> {
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap());
    // retrieve transactions with tag_id or its children
    let transactions = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, remark, wallet_id, tag_id, amount, time
        FROM transactions
        WHERE tag_id IN (
            WITH RECURSIVE tag_tree(id) AS (
                SELECT id FROM tags WHERE id = $1
                UNION ALL
                SELECT tags.id FROM tags JOIN tag_tree ON tags.parent_id = tag_tree.id
            )
            SELECT id FROM tag_tree
        ) AND date between $2 and $3
        ORDER BY date DESC
        LIMIT $4 OFFSET $5
        "#)
        .bind(tag_id).bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .bind(page_size).bind(page * page_size)
        .fetch_all(db())
        .await?;
    info!("Retrieved {} transactions with tag `{}`.", transactions.len(), tag_id);
    Ok(transactions)
}

#[tauri::command]
pub async fn delete_transaction(id: u32) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM transactions
        WHERE id = $1
        "#)
        .bind(id)
        .execute(db())
        .await?;
    info!("Transaction deleted");
    Ok(())
}