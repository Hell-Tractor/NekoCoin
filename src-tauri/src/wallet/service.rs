use sqlx::{Executor, Sqlite};
use tracing::{debug, info};

use crate::{sql::db, Result};

use super::Wallet;

pub async fn get_wallet_by_id(id: u32) -> Result<Wallet> {
    let wallet = sqlx::query_as::<_, Wallet>(
        r#"
        SELECT id, name, remark, balance, currency, color, icon
        FROM wallets
        WHERE id = $1
        "#)
        .bind(id)
        .fetch_one(db())
        .await?;
    debug!("Get wallet `{}`.", wallet.id);
    Ok(wallet)
}

#[allow(dead_code)]
pub async fn update_wallet<'c, E>(executor: E, wallet: Wallet) -> Result<()>
    where E: Executor<'c, Database = Sqlite>
{
    sqlx::query(
        r#"
        UPDATE wallets
        SET name = $1, remark = $2, balance = $3, currency = $4, color = $5, icon = $6
        WHERE id = $7
        "#)
        .bind(wallet.name).bind(wallet.remark).bind::<i32>(wallet.balance.balance.into()).bind(wallet.balance.get_currency()).bind(wallet.color).bind(wallet.icon).bind(wallet.id)
        .execute(executor)
        .await?;
    info!("Wallet `{}` updated", wallet.id);
    Ok(())
}

pub async fn modify_currency<'c, E>(executor: E, wallet_id: u32, amount: i32) -> Result<()>
    where E: Executor<'c, Database = Sqlite>
{
    sqlx::query(
        r#"
        UPDATE wallets
        SET balance = balance + ($1)
        WHERE id = $2
        "#)
        .bind(amount).bind(wallet_id)
        .execute(executor)
        .await?;
    info!("Wallet `{}` balance modified", wallet_id);
    Ok(())
}