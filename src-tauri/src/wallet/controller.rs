use tracing::{debug, info};

use crate::sql::db;

use super::Wallet;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
type Result<T> = std::result::Result<T, Error>;

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
pub async fn retrieve_wallets() -> Result<Vec<Wallet>> {
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

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}