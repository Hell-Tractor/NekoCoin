use sqlx::Row;
use tracing::{debug, info};

use crate::sql::db;
use crate::tag::TagKind;
use crate::transaction::service::{delete_transaction, revert_currency};
use crate::Result;

use super::vo::UpdateWalletVo;
use super::Wallet;

#[tauri::command]
pub async fn create_wallet(name: String, remark: String, balance: u32, currency_code: String, color: String, icon: String) -> Result<()> {
    debug!("Creating wallet: {} {} {} {} {} {}", name, remark, balance, currency_code, color, icon);
    sqlx::query(
        r#"
        INSERT INTO wallets (name, remark, balance, currency_code, color, icon)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#)
        .bind(name).bind(remark).bind(balance).bind(currency_code).bind(color).bind(icon)
        .execute(db())
        .await?;
    info!("Wallet created");
    Ok(())
}

#[tauri::command]
pub async fn update_wallet(vo: UpdateWalletVo) -> Result<()> {
    debug!("Updating wallet(id = {})", vo.id);
    sqlx::query(
        r#"
        UPDATE wallets
        SET name = $1, remark = $2, balance = $3, color = $4, icon = $5
        WHERE id = $6
        "#)
        .bind(vo.name).bind(vo.remark).bind(vo.balance).bind(vo.color).bind(vo.icon).bind(vo.id)
        .execute(db())
        .await?;
    info!("Wallet(id = {}) updated", vo.id);
    Ok(())
}

#[tauri::command]
pub async fn get_wallet_by_id(id: u32) -> Result<Wallet> {
    super::service::get_wallet_by_id(id).await
}

#[tauri::command]
pub async fn retrieve_wallets() -> Result<Vec<Wallet>> {
    debug!("Retrieving wallets...");
    let wallets = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, name, remark, balance, currency_code, color, icon
        FROM wallets
        "#)
        .fetch_all(db())
        .await?;
    info!("Retrieved {} wallets.", wallets.len());
    Ok(wallets)
}

#[tauri::command]
pub async fn delete_wallet(id: u32) -> Result<()> {
    debug!("Deleting wallet(id = {})", id);
    let mut tx = db().begin().await?;
    let owned_ids = sqlx::query(
        r#"
        SELECT id FROM transactions
        WHERE wallet_id = $1 OR to_wallet_id = $1
        "#)
        .bind(id)
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|row| row.get::<u32, _>("id"))
        .collect::<Vec<_>>();
    for transaction_id in owned_ids {
        delete_transaction(&mut *tx, transaction_id).await?;
    }

    let split_rows = sqlx::query(
        r#"
        SELECT transactions.id AS transaction_id, transactions.amount, ts.id AS split_id, ts.expense, ts.receive_wallet_id
        FROM transactions
        JOIN transaction_splits ts ON transactions.split_id = ts.id
        WHERE ts.receive_wallet_id = $1
        "#)
        .bind(id)
        .fetch_all(&mut *tx)
        .await?;
    for row in split_rows {
        let transaction_id: u32 = row.get("transaction_id");
        let split_id: u32 = row.get("split_id");
        let amount: i32 = row.get("amount");
        let expense: i32 = row.get("expense");
        let receive_wallet_id: u32 = row.get("receive_wallet_id");
        revert_currency(&mut *tx, &TagKind::Income, receive_wallet_id, None, amount - expense).await?;
        sqlx::query("UPDATE transactions SET split_id = NULL WHERE id = $1")
            .bind(transaction_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM transaction_splits WHERE id = $1")
            .bind(split_id)
            .execute(&mut *tx)
            .await?;
    }

    sqlx::query("DELETE FROM wallets WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    info!("Wallet(id = {}) deleted", id);
    Ok(())
}

#[tauri::command]
pub async fn get_sum_balance(currency_code: String) -> Result<i32> {
    let sum = sqlx::query_scalar::<_, i32>(
        r#"
        SELECT SUM(balance) FROM wallets
        WHERE currency_code = $1
        "#)
        .bind(currency_code)
        .fetch_one(db())
        .await?;
    info!("Sum of all wallets: {}", sum);
    Ok(sum)
}