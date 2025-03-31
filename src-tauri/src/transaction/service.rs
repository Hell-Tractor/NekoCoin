use tracing::{debug, info};

use crate::sql::db;

use super::{Transaction, TransactionSplit};

pub async fn get_transaction_by_id(id: u32) -> crate::Result<Transaction> {
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

pub async fn get_split_by_id(id: u32) -> crate::Result<TransactionSplit> {
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