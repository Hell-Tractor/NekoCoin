use chrono::{NaiveDate, NaiveDateTime};
use tracing::{debug, info};
use sqlx::{Row, SqliteConnection};

use crate::sql::db;
use crate::tag::TagKind;
use crate::{tag, wallet, Error, Result};

use super::dto::{BalanceWithTypeDto, TransactionDto};
use super::Transaction;

async fn modify_currency(executor: &mut SqliteConnection, tag_kind: &TagKind, wallet_id: u32, to_wallet_id: Option<u32>, amount: i32) -> Result<()> {
    debug!("Modifying currency...");
    match *tag_kind {
        tag::TagKind::Expense => wallet::service::modify_currency(executor, wallet_id, -amount).await?,
        tag::TagKind::Income => wallet::service::modify_currency(executor, wallet_id, amount).await?,
        tag::TagKind::Transfer => {
            if to_wallet_id.is_none() {
                return Err(Error::InvalidParameter("to_wallet_id is required for transfer".to_string()));
            }
            let to_wallet_id = to_wallet_id.unwrap();
            if to_wallet_id == wallet_id {
                return Err(Error::InvalidParameter("to_wallet_id cannot be the same as wallet_id".to_string()));
            }
            let from_wallet = wallet::service::get_wallet_by_id(wallet_id).await?;
            let to_wallet = wallet::service::get_wallet_by_id(to_wallet_id).await?;
            if from_wallet.balance.get_currency() != to_wallet.balance.get_currency() {
                return Err(Error::InvalidParameter("from_wallet and to_wallet must have the same currency".to_string()));
            }
            wallet::service::modify_currency(&mut *executor, wallet_id, -amount).await?;
            wallet::service::modify_currency(executor, to_wallet_id, amount).await?;
        }
    }
    Ok(())
}

async fn revert_currency(executor: &mut SqliteConnection, tag_kind: &TagKind, wallet_id: u32, to_wallet_id: Option<u32>, amount: i32) -> Result<()> {
    debug!("Reverting currency...");
    match *tag_kind {
        tag::TagKind::Expense => wallet::service::modify_currency(executor, wallet_id, amount).await?,
        tag::TagKind::Income => wallet::service::modify_currency(executor, wallet_id, -amount).await?,
        tag::TagKind::Transfer => {
            // if tag.kind == TagKind::Transfer, to_wallet_id should not be None
            // do not handle the case where to_wallet_id is None for now
            // do not check if wallets have same currency neither
            let to_wallet_id = to_wallet_id.unwrap();
            wallet::service::modify_currency(&mut *executor, to_wallet_id, -amount).await?;
            wallet::service::modify_currency(executor, wallet_id, amount).await?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn create_transaction(remark: String, wallet_id: u32, tag_id: u32, amount: i32, time: String, to_wallet_id: Option<u32>) -> Result<()> {
    debug!("Creating transaction: {} {} {} {} {}", remark, wallet_id, tag_id, amount, time);
    let time = NaiveDateTime::parse_from_str(&time, super::DATETIME_FORMAT)?;
    let mut tx = db().begin().await?;

    let tag = tag::service::get_tag_by_id(tag_id).await?;
    modify_currency(&mut *tx, &tag.kind, wallet_id, to_wallet_id, amount).await?;

    sqlx::query(
        r#"
        INSERT INTO transactions (remark, wallet_id, to_wallet_id, tag_id, amount, time)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#)
        .bind(remark).bind(wallet_id).bind(to_wallet_id).bind(tag_id).bind(amount).bind(time.format(super::DATETIME_FORMAT).to_string())
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    info!("Transaction created");
    Ok(())
}

#[tauri::command]
pub async fn update_transaction(mut transaction: Transaction) -> Result<()> {
    debug!("Updating transaction(id = {})", transaction.id);
    let mut tx = db().begin().await?;
    let mut old_transaction = super::service::get_transaction_by_id(transaction.id).await?;
    let old_tag_kind = old_transaction.get_tag().await?.kind.clone(); // * remove clone in the future
    revert_currency(&mut *tx, &old_tag_kind, old_transaction.wallet_id, old_transaction.to_wallet_id, old_transaction.amount).await?;
    let tag_kind = transaction.get_tag().await?.kind.clone();   // * remove clone in the future
    modify_currency(&mut *tx, &tag_kind, transaction.wallet_id, transaction.to_wallet_id, transaction.amount).await?;
    sqlx::query(
        r#"
        UPDATE transactions
        SET remark = $1, wallet_id = $2, to_wallet_id = $3, tag_id = $4, amount = $5, time = $6
        WHERE id = $7
        "#)
        .bind(transaction.remark).bind(transaction.wallet_id).bind(transaction.to_wallet_id).bind(transaction.tag_id).bind(transaction.amount).bind(transaction.time.format(super::DATETIME_FORMAT).to_string()).bind(transaction.id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    info!("Transaction(id = {}) updated", transaction.id);
    Ok(())
}

#[tauri::command]
pub async fn retrieve_transactions(begin: Option<NaiveDate>, end: Option<NaiveDate>, page: u32, page_size: u32) -> Result<Vec<TransactionDto>> {
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    let transactions = sqlx::query(
        r#"
        SELECT transactions.id, transactions.remark, wallets.name AS wallet_name, to_wallets.name AS to_wallet_name, wallets.currency, transactions.tag_id, transactions.amount, transactions.time
        FROM transactions
        JOIN wallets ON transactions.wallet_id = wallets.id
        LEFT JOIN wallets AS to_wallets ON transactions.to_wallet_id = to_wallets.id
        WHERE time between $1 and $2
        ORDER BY time DESC
        LIMIT $3 OFFSET $4
        "#)
        .bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .bind(page_size).bind(page * page_size)
        .fetch_all(db())
        .await?;
    let transactions = transactions.iter().map(|row| TransactionDto::try_from_row(row));
    let transactions = futures::future::try_join_all(transactions).await?;
    info!("Retrieved {} transactions.", transactions.len());
    Ok(transactions)
}

#[tauri::command]
pub async fn retrieve_transactions_in_wallet(wallet_id: u32, begin: Option<NaiveDate>, end: Option<NaiveDate>, page: u32, page_size: u32) -> Result<Vec<TransactionDto>> {
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    let transactions = sqlx::query(
        r#"
        SELECT transactions.id, transactions.remark, wallets.name AS wallet_name, to_wallets.name AS to_wallet_name, wallets.currency, transactions.tag_id, transactions.amount, transactions.time
        FROM transactions
        JOIN wallets ON transactions.wallet_id = wallet.id
        LEFT JOIN wallets AS to_wallets ON transactions.to_wallet_id = to_wallets.id
        WHERE wallet_id = $1 AND time between $2 and $3
        ORDER BY time DESC
        LIMIT $4 OFFSET $5
        "#)
        .bind(wallet_id).bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .bind(page_size).bind(page * page_size)
        .fetch_all(db())
        .await?;
    let transactions = transactions.iter().map(|row| TransactionDto::try_from_row(row));
    let transactions = futures::future::try_join_all(transactions).await?;
    info!("Retrieved {} transactions in wallet `{}`.", transactions.len(), wallet_id);
    Ok(transactions)
}

#[tauri::command]
pub async fn retrieve_transactions_with_tag(tag_id: u32, begin: Option<NaiveDate>, end: Option<NaiveDate>, page: u32, page_size: u32) -> Result<Vec<TransactionDto>> {
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    // retrieve transactions with tag_id or its children
    let transactions = sqlx::query(
        r#"
        SELECT transactions.id, transactions.remark, wallets.name AS wallet_name, to_wallets.name AS to_wallet_name, wallets.currency, transactions.tag_id, transactions.amount, transactions.time
        FROM transactions
        JOIN wallets ON transactions.wallet_id = wallet.id
        LEFT JOIN wallets AS to_wallets ON transactions.to_wallet_id = to_wallets.id
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
    let transactions = transactions.iter().map(|row| TransactionDto::try_from_row(row));
    let transactions = futures::future::try_join_all(transactions).await?;
    info!("Retrieved {} transactions with tag `{}`.", transactions.len(), tag_id);
    Ok(transactions)
}

#[tauri::command]
pub async fn delete_transaction(id: u32) -> Result<()> {
    debug!("Deleting transaction: {}", id);
    let mut transaction = super::service::get_transaction_by_id(id).await?;
    let mut tx = db().begin().await?;
    let tag_kind = transaction.get_tag().await?.kind.clone(); // * remove clone in the future
    revert_currency(&mut *tx, &tag_kind, transaction.wallet_id, transaction.to_wallet_id, transaction.amount).await?;
    // currency have been reverted, now delete transaction
    sqlx::query(
        r#"
        DELETE FROM transactions
        WHERE id = $1
        "#)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    info!("Transaction deleted");
    Ok(())
}

#[tauri::command]
pub async fn get_sum_balance_with_type(currency: String, begin: Option<NaiveDate>, end: Option<NaiveDate>) -> Result<BalanceWithTypeDto> {
    debug!("Getting sum of transactions with kind...");
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    let begin = begin.and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap());
    let end = end.and_hms_opt(23, 59, 59).unwrap();
    let sum = sqlx::query(
        r#"
        SELECT SUM(amount), tags.kind FROM transactions
        JOIN tags ON transactions.tag_id = tags.id
        JOIN wallets ON transactions.wallet_id = wallets.id
        WHERE tags.kind != $1 AND wallets.currency = $2 AND time between $3 and $4
        GROUP BY tags.kind
        "#)
        .bind(TagKind::Transfer as u8).bind(currency).bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .fetch_all(db())
        .await?;
    let mut result = BalanceWithTypeDto::default();
    for row in sum {
        let amount: i32 = row.get(0);
        let kind: TagKind = row.get(1);
        match kind {
            TagKind::Expense => result.expense = amount,
            TagKind::Income => result.income = amount,
            _ => unreachable!(),
        }
    }
    info!("Sum of balance with type in transactions: {:?}", result);
    Ok(result)
}