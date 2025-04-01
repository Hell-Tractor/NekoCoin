use sqlx::SqliteConnection;
use tracing::{debug, info};

use crate::{sql::db, tag::{self, TagKind}, wallet, Error, Result};

use super::{Transaction, TransactionSplit};

pub async fn get_transaction_by_id(id: u32) -> Result<Transaction> {
    debug!("Getting transaction with id: {}", id);
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, remark, wallet_id, to_wallet_id, tag_id, amount, time, split_id
        FROM transactions
        WHERE transactions.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(db())
    .await?;
    info!("Transaction(id = {}) found", id);
    Ok(transaction)
}

pub async fn get_split_by_id(id: u32) -> Result<TransactionSplit> {
    debug!("Getting transaction split with id: {}", id);
    let transaction_split = sqlx::query_as::<_, TransactionSplit>(
        r#"
        SELECT id, count, expense, recieve_wallet_id
        FROM transaction_splits
        WHERE transaction_splits.id = $1
        "#,
    ).bind(id)
    .fetch_one(db())
    .await?;
    info!("TransactionSplit(id = {}) found", id);
    Ok(transaction_split)
}

pub async fn delete_transaction(executor: &mut SqliteConnection, id: u32) -> Result<()> {
    let mut transaction = super::service::get_transaction_by_id(id).await?;
    let tag_kind = transaction.get_tag().await?.kind.clone(); // * remove clone in the future
    // revert currency payed from original wallet
    revert_currency(&mut *executor, &tag_kind, transaction.wallet_id, transaction.to_wallet_id, transaction.amount).await?;
    // revert income from transaction split
    let amount = transaction.amount; // * remove clone in the future
    if let Some(split) = transaction.get_split().await? {
        revert_currency(&mut *executor, &TagKind::Income, split.recieve_wallet_id, None, amount - split.expense).await?;
    }
    // currency have been reverted, now delete transaction
    sqlx::query(
        r#"
        DELETE FROM transactions
        WHERE id = $1
        "#)
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn modify_currency(executor: &mut SqliteConnection, tag_kind: &TagKind, wallet_id: u32, to_wallet_id: Option<u32>, amount: i32) -> Result<()> {
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

pub async fn revert_currency(executor: &mut SqliteConnection, tag_kind: &TagKind, wallet_id: u32, to_wallet_id: Option<u32>, amount: i32) -> Result<()> {
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