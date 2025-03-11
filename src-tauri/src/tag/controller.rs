use tracing::{debug, info};

use crate::sql::db;

use super::{Tag, TagKind};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}
type Result<T> = std::result::Result<T, Error>;

#[tauri::command]
pub async fn create_tag(name: String, remark: String, color: String, icon: String, kind: TagKind, parent_id: Option<u32>) -> Result<()> {
    debug!("Creating tag `{}` with parent_id=`{:?}` in type `{:?}`", name, parent_id, kind);
    sqlx::query(
        r#"
        INSERT INTO tags (name, remark, color, icon, kind, parent_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#)
        .bind(name).bind(remark).bind(color).bind(icon).bind(kind as u8).bind(parent_id)
        .execute(db())
        .await?;
    info!("Tag created");
    Ok(())
}

#[tauri::command]
pub async fn retrieve_tags(filter: String, kind: Option<TagKind>) -> Result<Vec<Tag>> {
    let keywords = filter.trim().replace("  ", " ").split_whitespace().map(|word| word.into()).collect::<Vec<String>>();
    let tags = if kind.is_none() {
        sqlx::query_as::<_, Tag>(
            r#"
            SELECT * FROM tags
            WHERE name LIKE '%' || $1 || '%'
            "#)
            .bind(keywords.join("%"))
            .fetch_all(db())
            .await?
    } else {
        sqlx::query_as::<_, Tag>(
            r#"
            SELECT * FROM tags
            WHERE name LIKE '%' || $1 || '%'
            AND kind = $2
            "#)
            .bind(keywords.join("%")).bind(kind.unwrap() as u8)
            .fetch_all(db())
            .await?
    };
    info!("Retrieved {} tags.", tags.len());
    Ok(tags)
}

#[tauri::command]
pub async fn delete_tag(id: u32) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM tags
        WHERE id = $1
        "#)
        .bind(id)
        .execute(db())
        .await?;
    info!("Tag deleted");
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