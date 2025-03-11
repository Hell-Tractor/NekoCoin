use crate::sql::db;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
type Result<T> = std::result::Result<T, Error>;

#[tauri::command]
pub async fn create_wallet(name: String, remark: String, balance: u32, currency: String) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO wallets (name, remark, balance, currency)
        VALUES ($1, $2, $3, $4)
        "#)
        .bind(name).bind(remark).bind(balance).bind(currency)
        .execute(db())
        .await?;
    Ok(())
}

// #[tauri::command]
// pub async fn retrieve_wallets(page: u32, page_size: u32) -> Result<Vec<Wallet>> {
//     let wallets = sqlx::query_as::<_, Wallet>(
//         r#"
//         SELECT id, name, remark, balance, currency
//         FROM wallets
//         LIMIT $1 OFFSET $2
//         "#)
//         .bind(page_size).bind(page * page_size)
//         .fetch_all(db())
//         .await?;
//     Ok(wallets)
// }

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}