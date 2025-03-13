use tracing::info;

use crate::{sql::db, Result};

use super::Tag;

pub async fn get_tag_by_id(id: u32) -> Result<Tag> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"
        SELECT *
        FROM tags
        WHERE id = $1
        "#)
        .bind(id)
        .fetch_one(db())
        .await?;
    info!("Get tag `{}`.", tag.id);
    Ok(tag)
}