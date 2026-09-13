use tracing::{debug, info};

use crate::sql::db;
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