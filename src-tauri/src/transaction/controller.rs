use chrono::{NaiveDate, NaiveDateTime};
use tracing::{debug, info};
use sqlx::{Row, SqliteConnection};

use crate::sql::db;
use crate::tag::TagKind;
use crate::{tag, wallet, Error, Result};

use super::dto::{BalanceWithTypeDto, TransactionDto};
use super::vo::{CreateTransactionVo, TransactionVo};

async fn modify_currency(executor: &mut SqliteConnection, tag_kind: &TagKind, wallet_id: u32, to_wallet_id: Option<u32>, amount: i32) -> Result<()> {
    debug!("Modifying currency with {:?} for wallet(id = {}) to wallet(id = {:?})...", tag_kind, wallet_id, to_wallet_id);
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
    debug!("Reverting currency with {:?} for wallet(id = {}) to wallet(id = {:?})...", tag_kind, wallet_id, to_wallet_id);
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
pub async fn create_transaction(vo: CreateTransactionVo) -> Result<()> {
    debug!("Creating transaction: {:?}", vo);
    let time = NaiveDateTime::parse_from_str(&vo.time, super::DATETIME_FORMAT)?;
    let mut tx = db().begin().await?;

    let tag = tag::service::get_tag_by_id(vo.tag_id).await?;
    modify_currency(&mut *tx, &tag.kind, vo.wallet_id, vo.to_wallet_id, vo.amount).await?;

    let split_id = if let Some(split) = vo.split {
        debug!("Creating transaction split: {:?}", split);
        if tag.kind != TagKind::Expense {
            return Err(Error::InvalidParameter("split is only allowed for expense".to_string()));
        }
        // receive wallet should have same currency as original wallet
        let receive_wallet = wallet::service::get_wallet_by_id(split.recieve_wallet_id).await?;
        let original_wallet = wallet::service::get_wallet_by_id(vo.wallet_id).await?;
        if receive_wallet.balance.get_currency() != original_wallet.balance.get_currency() {
            return Err(Error::InvalidParameter("receive wallet and original wallet must have same currency".to_string()));
        }
        modify_currency(&mut *tx, &TagKind::Income, split.recieve_wallet_id, None, vo.amount - split.expense).await?;
        let split_id: u32 = sqlx::query(
            r#"
            INSERT INTO transaction_splits (count, expense, recieve_wallet_id)
            VALUES ($1, $2, $3)
            RETURNING id
            "#)
            .bind(split.count).bind(split.expense).bind(split.recieve_wallet_id)
            .fetch_one(&mut *tx)
            .await?
            .get(0);
        // sqlx::query(
        //     r#"
        //     INSERT INTO transactions (remark, wallet_id, to_wallet_id, tag_id, amount, time, split_id)
        //     VALUES ($1, $2, $3, $4, $5, $6, $7)
        //     "#)
        //     .bind(vo.remark).bind(vo.wallet_id).bind(vo.to_wallet_id).bind(vo.tag_id).bind(split.expense).bind(time.format(super::DATETIME_FORMAT).to_string()).bind(split_id)
        //     .execute(&mut *tx)
        //     .await?;
        debug!("Transaction split(id = {}) created.", split_id);
        Some(split_id)
    } else {
        None
    };

    sqlx::query(
        r#"
        INSERT INTO transactions (remark, wallet_id, to_wallet_id, tag_id, amount, time, split_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#)
        .bind(vo.remark).bind(vo.wallet_id).bind(vo.to_wallet_id).bind(vo.tag_id).bind(vo.amount).bind(time.format(super::DATETIME_FORMAT).to_string()).bind(split_id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;

    info!("Transaction created");
    Ok(())
}

#[tauri::command]
pub async fn update_transaction(vo: TransactionVo) -> Result<()> {
    debug!("Updating transaction(id = {})", vo.id);
    let mut tx = db().begin().await?;
    // let mut bo: Transaction = vo.into();
    let mut old_transaction = super::service::get_transaction_by_id(vo.id).await?;
    debug!("Old transaction: {:?}", old_transaction);
    let old_tag_kind = old_transaction.get_tag().await?.kind.clone(); // * remove clone in the future
    revert_currency(&mut *tx, &old_tag_kind, old_transaction.wallet_id, old_transaction.to_wallet_id, old_transaction.amount).await?;
    let tag_kind = tag::service::get_tag_by_id(vo.tag_id).await?.kind;  // * remove clone in the future
    modify_currency(&mut *tx, &tag_kind, vo.wallet_id, vo.to_wallet_id, vo.amount).await?;

    let split_id = if let Some(split) = vo.split {
        debug!("Updating with new transaction split: {:?}", split);
        if tag_kind != TagKind::Expense {
            return Err(Error::InvalidParameter("split is only allowed for expense".to_string()));
        }
        let receive_wallet = wallet::service::get_wallet_by_id(split.recieve_wallet_id).await?;
        let original_wallet = wallet::service::get_wallet_by_id(vo.wallet_id).await?;
        // receive wallet should have same currency as original wallet
        if receive_wallet.balance.get_currency() != original_wallet.balance.get_currency() {
            return Err(Error::InvalidParameter("receive wallet and original wallet must have same currency".to_string()));
        }
        // if no split id provided, it's a new split
        let split_id = if split.id.is_none() {
            // then it should not have old split
            if old_transaction.split_id.is_some() {
                return Err(Error::InvalidParameter(format!("data mismatch: split id not provided but found old split(id={})", old_transaction.split_id.unwrap())));
            }
            // modify currency for new split
            modify_currency(&mut *tx, &TagKind::Income, split.recieve_wallet_id, None, vo.amount - split.expense).await?;
            let split_id: u32 = sqlx::query(
                r#"
                INSERT INTO transaction_splits (count, expense, recieve_wallet_id)
                VALUES ($1, $2, $3)
                RETURNING id
                "#)
                .bind(split.count).bind(split.expense).bind(split.recieve_wallet_id)
                .fetch_one(&mut *tx)
                .await?
                .get(0);
            Some(split_id)
        } else {    // it's a old split
            // old split should not be None
            if old_transaction.split_id.is_none() {
                return Err(Error::InvalidParameter("data mismatch: split id provided but not found old split".to_string()));
            }
            // split id should be same as old transaction split id
            if split.id.unwrap() != old_transaction.split_id.unwrap() {
                return Err(Error::InvalidParameter("data mismatch: split id not match".to_string()));
            }
            // revert old split currency
            let old_amount = old_transaction.amount; // * remove clone in the future
            let old_split = old_transaction.get_split().await?.unwrap(); // split with given id should always exists
            revert_currency(&mut *tx, &TagKind::Income, old_split.recieve_wallet_id, None, old_amount - old_split.expense).await?;
            // modify currency for new split
            modify_currency(&mut *tx, &TagKind::Income, split.recieve_wallet_id, None, vo.amount - split.expense).await?;
            // update split
            sqlx::query(
                r#"
                UPDATE transaction_splits
                SET count = $1, expense = $2, recieve_wallet_id = $3
                WHERE id = $4
                "#)
                .bind(split.count).bind(split.expense).bind(split.recieve_wallet_id).bind(split.id.unwrap())
                .execute(&mut *tx)
                .await?;
            Some(split.id.unwrap())
        };
        debug!("Transaction split(id = {}) updated.", split_id.unwrap());
        split_id
    } else { // no split provided
        debug!("Updating without transaction split");
        // if old transaction has split, it should be removed
        if old_transaction.split_id.is_some() {
            debug!("Removing old transaction split: {}", old_transaction.split_id.unwrap());
            // revert old split currency
            let old_amount = old_transaction.amount; // * remove clone in the future
            let old_split = old_transaction.get_split().await?.unwrap(); // split with given id should always exists
            revert_currency(&mut *tx, &TagKind::Income, old_split.recieve_wallet_id, None, old_amount - old_split.expense).await?;
            // delete split
            sqlx::query(
                r#"
                DELETE FROM transaction_splits
                WHERE id = $1
                "#)
                .bind(old_transaction.split_id.unwrap())
                .execute(&mut *tx)
                .await?;
        }
        None
    };

    sqlx::query(
        r#"
        UPDATE transactions
        SET remark = $1, wallet_id = $2, to_wallet_id = $3, tag_id = $4, amount = $5, time = $6, split_id = $7
        WHERE id = $8
        "#)
        .bind(vo.remark).bind(vo.wallet_id).bind(vo.to_wallet_id).bind(vo.tag_id).bind(vo.amount).bind(vo.time).bind(split_id).bind(vo.id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    info!("Transaction(id = {}) updated", vo.id);
    Ok(())
}

#[tauri::command]
pub async fn retrieve_transactions(begin: Option<NaiveDate>, end: Option<NaiveDate>, page: u32, page_size: u32) -> Result<Vec<TransactionDto>> {
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    let transactions = sqlx::query(
        r#"
        SELECT transactions.id, transactions.remark, wallets.name AS wallet_name, to_wallets.name AS to_wallet_name, wallets.currency, transactions.tag_id, transactions.amount, transactions.time, transactions.split_id
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
        SELECT transactions.id, transactions.remark, wallets.name AS wallet_name, to_wallets.name AS to_wallet_name, wallets.currency, transactions.tag_id, transactions.amount, transactions.time, transactions.split_id
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
        SELECT transactions.id, transactions.remark, wallets.name AS wallet_name, to_wallets.name AS to_wallet_name, wallets.currency, transactions.tag_id, transactions.amount, transactions.time, transactions.split_id
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
    // revert currency payed from original wallet
    revert_currency(&mut *tx, &tag_kind, transaction.wallet_id, transaction.to_wallet_id, transaction.amount).await?;
    // revert income from transaction split
    let amount = transaction.amount; // * remove clone in the future
    if let Some(split) = transaction.get_split().await? {
        revert_currency(&mut *tx, &TagKind::Income, split.recieve_wallet_id, None, amount - split.expense).await?;
    }
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