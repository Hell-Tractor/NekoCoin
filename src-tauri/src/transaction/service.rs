use tracing::{debug, info};

use crate::sql::db;

use super::Transaction;

pub async fn get_transaction_by_id(id: u32) -> crate::Result<Transaction> {
    debug!("Getting transaction with id: {}", id);
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        SELECT id, remark, wallet_id, to_wallet_id, tag_id, amount, time
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