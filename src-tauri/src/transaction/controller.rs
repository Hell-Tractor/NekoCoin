use chrono::{NaiveDate, NaiveDateTime};
use tracing::{debug, info};
use sqlx::Row;

use crate::sql::db;
use crate::tag::TagKind;
use crate::{tag, wallet, Error, Result};

use super::dto::{BalanceWithTypeDto, SummaryByTagDto, TransactionDto};
use super::service::{modify_currency, revert_currency};
use super::vo::{CreateTransactionVo, TransactionVo};

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
        let receive_wallet = wallet::service::get_wallet_by_id(split.receive_wallet_id).await?;
        let original_wallet = wallet::service::get_wallet_by_id(vo.wallet_id).await?;
        if receive_wallet.balance.get_currency() != original_wallet.balance.get_currency() {
            return Err(Error::InvalidParameter("receive wallet and original wallet must have same currency".to_string()));
        }
        modify_currency(&mut *tx, &TagKind::Income, split.receive_wallet_id, None, vo.amount - split.expense).await?;
        let split_id: u32 = sqlx::query(
            r#"
            INSERT INTO transaction_splits (count, expense, receive_wallet_id)
            VALUES ($1, $2, $3)
            RETURNING id
            "#)
            .bind(split.count).bind(split.expense).bind(split.receive_wallet_id)
            .fetch_one(&mut *tx)
            .await?
            .get(0);
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
        let receive_wallet = wallet::service::get_wallet_by_id(split.receive_wallet_id).await?;
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
            modify_currency(&mut *tx, &TagKind::Income, split.receive_wallet_id, None, vo.amount - split.expense).await?;
            let split_id: u32 = sqlx::query(
                r#"
                INSERT INTO transaction_splits (count, expense, receive_wallet_id)
                VALUES ($1, $2, $3)
                RETURNING id
                "#)
                .bind(split.count).bind(split.expense).bind(split.receive_wallet_id)
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
            revert_currency(&mut *tx, &TagKind::Income, old_split.receive_wallet_id, None, old_amount - old_split.expense).await?;
            // modify currency for new split
            modify_currency(&mut *tx, &TagKind::Income, split.receive_wallet_id, None, vo.amount - split.expense).await?;
            // update split
            sqlx::query(
                r#"
                UPDATE transaction_splits
                SET count = $1, expense = $2, receive_wallet_id = $3
                WHERE id = $4
                "#)
                .bind(split.count).bind(split.expense).bind(split.receive_wallet_id).bind(split.id.unwrap())
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
            revert_currency(&mut *tx, &TagKind::Income, old_split.receive_wallet_id, None, old_amount - old_split.expense).await?;
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
        JOIN wallets ON transactions.wallet_id = wallets.id
        LEFT JOIN transaction_splits ts ON transactions.split_id = ts.id
        LEFT JOIN wallets AS to_wallets ON transactions.to_wallet_id = to_wallets.id
        WHERE (wallet_id = $1 OR to_wallet_id = $1 OR ts.receive_wallet_id = $1)
            AND time between $2 and $3
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
        JOIN wallets ON transactions.wallet_id = wallets.id
        LEFT JOIN wallets AS to_wallets ON transactions.to_wallet_id = to_wallets.id
        WHERE tag_id IN (
            WITH RECURSIVE tag_tree(id) AS (
                SELECT id FROM tags WHERE id = $1
                UNION ALL
                SELECT tags.id FROM tags JOIN tag_tree ON tags.parent_id = tag_tree.id
            )
            SELECT id FROM tag_tree
        ) AND time between $2 and $3
        ORDER BY time DESC
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
    let mut tx = db().begin().await?;
    super::service::delete_transaction(&mut *tx, id).await?;
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
    // TODO: rewrite sql
    let result = sqlx::query_as::<_, BalanceWithTypeDto>(
        r#"
        SELECT
            SUM(CASE WHEN tags.kind = $1 THEN COALESCE(ts.expense, amount) ELSE 0 END) AS expense,
            SUM(CASE WHEN tags.kind = $2 THEN amount ELSE 0 END) AS income
        FROM transactions
        LEFT JOIN transaction_splits ts ON transactions.split_id = ts.id
        JOIN tags ON transactions.tag_id = tags.id
        JOIN wallets ON transactions.wallet_id = wallets.id
        WHERE wallets.currency = $3 AND time between $4 and $5
        "#)
        .bind(TagKind::Expense as u8).bind(TagKind::Income as u8).bind(currency).bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .fetch_one(db())
        .await?;
    info!("Sum of balance with type in transactions: {:?}", result);
    Ok(result)
}

#[tauri::command]
pub async fn get_sum_balance_in_wallet(wallet_id: u32, begin: Option<NaiveDate>, end: Option<NaiveDate>) -> Result<BalanceWithTypeDto> {
    debug!("Getting sum of transactions in wallet(id = {})", wallet_id);
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    let result = sqlx::query_as::<_, BalanceWithTypeDto>(
        r#"
        SELECT
            SUM(CASE WHEN (tags.kind = $1 OR tags.kind = $3) AND wallet_id = $4 THEN amount ELSE 0 END) AS expense,
            SUM(CASE
                    WHEN (tags.kind = $2 AND wallet_id = $4) OR (tags.kind = $3 AND to_wallet_id = $4)
                    THEN amount ELSE 0
                END +
                CASE WHEN ts.receive_wallet_id = $4 THEN amount - ts.expense ELSE 0 END) AS income
        FROM transactions
        LEFT JOIN transaction_splits ts ON transactions.split_id = ts.id
        JOIN tags ON transactions.tag_id = tags.id
        WHERE time BETWEEN $5 AND $6
        "#)
        .bind(TagKind::Expense as u8).bind(TagKind::Income as u8).bind(TagKind::Transfer as u8)
        .bind(wallet_id).bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .fetch_one(db())
        .await?;
    info!("Sum of transactions in wallet: {:?}", result);
    Ok(result)
}

#[tauri::command]
pub async fn get_summary_by_tag_in_wallet(kind: TagKind, wallet_id: u32, begin: Option<NaiveDate>, end: Option<NaiveDate>) -> Result<Vec<SummaryByTagDto>> {
    debug!("Getting summary by tag(kind = {:?}) in wallet(id = {})", kind, wallet_id);
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    // 1. find tags' farthest parent id => root_id
    // 2. join tags by root_id
    // 3. group by root_id
    // 4. order by summary desc
    let result = if kind == TagKind::Expense {
        sqlx::query_as(
            r#"
            -- find all tags' farthest non null parent id => root_id
            WITH RECURSIVE tag_tree AS (
                SELECT t.id AS root_id, t.id AS id FROM tags t WHERE t.parent_id IS NULL
                UNION ALL
                SELECT tt.root_id, t.id FROM tag_tree tt JOIN tags t ON tt.id = t.parent_id
            )
            SELECT SUM(
                CASE WHEN (tag.kind = $4 OR tag.kind = $5) AND wallet_id = $3 THEN amount ELSE 0 END
            ) AS summary, tag.id, tag.name, tag.remark, tag.color, tag.icon, tag.kind, tag.parent_id
            FROM transactions
            JOIN tag_tree AS tt ON transactions.tag_id = tt.id
            JOIN tags AS tag ON tt.root_id = tag.id
            WHERE time BETWEEN $1 AND $2
            GROUP BY tt.root_id
            HAVING summary > 0
            ORDER BY summary DESC
            "#)
            .bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
            .bind(wallet_id).bind(TagKind::Expense as u8).bind(TagKind::Transfer as u8)
            .fetch_all(db())
            .await?
    } else if kind == TagKind::Income {
        sqlx::query_as(
            r#"
            -- find all tags' farthest non null parent id => root_id
            WITH RECURSIVE tag_tree AS (
                SELECT t.id AS root_id, t.id AS id FROM tags t WHERE t.parent_id IS NULL
                UNION ALL
                SELECT tt.root_id, t.id FROM tag_tree tt JOIN tags t ON tt.id = t.parent_id
            )
            SELECT SUM(
                CASE WHEN (tag.kind = $5 AND wallet_id = $3) OR (tag.kind = $6 AND to_wallet_id = $3) THEN amount ELSE 0 END +
                CASE WHEN (tag.kind = $4 AND ts.receive_wallet_id = $3) THEN amount - ts.expense ELSE 0 END
            ) AS summary, tag.id, tag.name, tag.remark, tag.color, tag.icon, tag.kind, tag.parent_id
            FROM transactions
            JOIN tag_tree AS tt ON transactions.tag_id = tt.id
            JOIN tags AS tag ON tt.root_id = tag.id
            LEFT JOIN transaction_splits ts ON transactions.split_id = ts.id
            WHERE time BETWEEN $1 AND $2
            GROUP BY tt.root_id
            HAVING summary > 0
            ORDER BY summary DESC
            "#)
            .bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
            .bind(wallet_id).bind(TagKind::Expense as u8).bind(TagKind::Income as u8).bind(TagKind::Transfer as u8)
            .fetch_all(db())
            .await?
    } else {
        return Err(Error::InvalidParameter("kind must be expense or income".to_string()));
    };
    info!("got summary by tag(length = {})", result.len());
    Ok(result)
}

#[tauri::command]
pub async fn get_summary_by_tag_with_tag(tag_id: u32, currency: String, begin: Option<NaiveDate>, end: Option<NaiveDate>) -> Result<Vec<SummaryByTagDto>> {
    debug!("Getting summary directly under tag(id = {})", tag_id);
    let begin = begin.unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).and_hms_opt(0, 0, 0).unwrap();
    let end = end.unwrap_or_else(|| NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()).and_hms_opt(23, 59, 59).unwrap();
    let result: Vec<SummaryByTagDto> = sqlx::query_as(
        r#"
        WITH RECURSIVE tag_tree AS (
            -- add tag(id = $3) itself
            SELECT id AS root_id, id FROM tags WHERE id = $3
            UNION
            -- add all children of tag(id = $3)
            SELECT t.id AS root_id, t.id AS id FROM tags t WHERE parent_id = $3
            UNION ALL
            -- find all children of tag(id = $3)'s children recursively, but not for tag(id = $3) itself
            SELECT tt.root_id, t.id FROM tag_tree tt JOIN tags t ON tt.id = t.parent_id WHERE tt.id != $3
        )
        SELECT SUM(amount) as summary, tag.id, tag.name, tag.remark, tag.color, tag.icon, tag.kind, tag.parent_id
        FROM transactions
        JOIN tag_tree tt ON transactions.tag_id = tt.id
        JOIN tags tag ON tt.root_id = tag.id
        JOIN wallets ON transactions.wallet_id = wallets.id
        WHERE time BETWEEN $1 AND $2 AND wallets.currency = $4
        GROUP BY tt.root_id
        "#)
        .bind(begin.format(super::DATETIME_FORMAT).to_string()).bind(end.format(super::DATETIME_FORMAT).to_string())
        .bind(tag_id).bind(currency)
        .fetch_all(db())
        .await?;
    info!("got summary under tag(length = {})", result.len());
    Ok(result)
}