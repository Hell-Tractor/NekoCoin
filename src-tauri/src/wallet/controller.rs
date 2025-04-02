use tracing::{debug, info};

use crate::sql::db;
use crate::Result;

use super::Wallet;

#[tauri::command]
pub async fn create_wallet(name: String, remark: String, balance: u32, currency: String, color: String, icon: String) -> Result<()> {
    debug!("Creating wallet: {} {} {} {} {} {}", name, remark, balance, currency, color, icon);
    sqlx::query(
        r#"
        INSERT INTO wallets (name, remark, balance, currency, color, icon)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#)
        .bind(name).bind(remark).bind(balance).bind(currency).bind(color).bind(icon)
        .execute(db())
        .await?;
    info!("Wallet created");
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
        SELECT id, name, remark, balance, currency, color, icon
        FROM wallets
        "#)
        .fetch_all(db())
        .await?;
    info!("Retrieved {} wallets.", wallets.len());
    Ok(wallets)
}

#[tauri::command]
pub async fn delete_wallet(id: u32) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM wallets
        WHERE id = $1
        "#)
        .bind(id)
        .execute(db())
        .await?;
    info!("Wallet deleted");
    Ok(())
}

#[tauri::command]
pub async fn get_sum_balance(currency: String) -> Result<u32> {
    let sum = sqlx::query_scalar::<_, u32>(
        r#"
        SELECT SUM(balance) FROM wallets
        WHERE currency = $1
        "#)
        .bind(currency)
        .fetch_one(db())
        .await?;
    info!("Sum of all wallets: {}", sum);
    Ok(sum)
}