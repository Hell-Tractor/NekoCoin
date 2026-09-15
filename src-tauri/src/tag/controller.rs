use tracing::{debug, info};
use sqlx::Row;

use crate::sql::db;
use crate::{activity, Error, Result};

use super::{vo::UpdateTagVo, Tag, TagKind};

async fn ensure_parent_kind(kind: TagKind, parent_id: Option<u32>) -> Result<()> {
    let Some(parent_id) = parent_id else {
        return Ok(());
    };
    let parent = super::service::get_tag_by_id(parent_id).await?;
    if parent.kind != kind {
        return Err(Error::InvalidTagType {
            given: parent.kind,
            allow: vec![kind],
        });
    }
    Ok(())
}

async fn ensure_parent_not_descendant(tag_id: u32, parent_id: Option<u32>) -> Result<()> {
    let Some(parent_id) = parent_id else {
        return Ok(());
    };
    if parent_id == tag_id {
        return Err(Error::InvalidParameter(format!(
            "tag(id = {tag_id}) cannot be its own parent"
        )));
    }
    let descendant_count = sqlx::query(
        r#"
        WITH RECURSIVE descendants(id) AS (
            SELECT id FROM tags WHERE parent_id = $1
            UNION ALL
            SELECT t.id FROM tags t JOIN descendants d ON t.parent_id = d.id
        )
        SELECT COUNT(*) FROM descendants WHERE id = $2
        "#)
        .bind(tag_id)
        .bind(parent_id)
        .fetch_one(db())
        .await?
        .get::<i64, _>(0);
    if descendant_count > 0 {
        return Err(Error::InvalidParameter(format!(
            "tag(id = {parent_id}) is a descendant of tag(id = {tag_id}) and cannot be its parent"
        )));
    }
    Ok(())
}

#[tauri::command]
pub async fn create_tag(name: String, remark: String, color: String, icon: String, kind: TagKind, parent_id: Option<u32>) -> Result<()> {
    debug!("Creating tag `{}` with parent_id=`{:?}` in type `{:?}`", name, parent_id, kind);
    ensure_parent_kind(kind, parent_id).await?;
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
pub async fn update_tag(vo: UpdateTagVo) -> Result<()> {
    debug!("Updating tag(id = {})", vo.id);
    let existing = super::service::get_tag_by_id(vo.id).await?;
    ensure_parent_kind(existing.kind, vo.parent_id).await?;
    ensure_parent_not_descendant(vo.id, vo.parent_id).await?;
    sqlx::query(
        r#"
        UPDATE tags
        SET name = $1, remark = $2, color = $3, icon = $4, parent_id = $5
        WHERE id = $6
        "#)
        .bind(vo.name).bind(vo.remark).bind(vo.color).bind(vo.icon).bind(vo.parent_id).bind(vo.id)
        .execute(db())
        .await?;
    info!("Tag(id = {}) updated", vo.id);
    Ok(())
}

#[tauri::command]
pub async fn get_tag_by_id(id: u32) -> Result<Tag> {
    super::service::get_tag_by_id(id).await
}

#[tauri::command]
pub async fn has_child_tag(id: u32) -> Result<bool> {
    let count = sqlx::query(
        r#"
        SELECT COUNT(*) FROM tags
        WHERE parent_id = $1
        "#)
        .bind(id)
        .fetch_one(db())
        .await?
        .get::<i64, _>(0);
    info!("Tag(id = {}) has {} child tags", id, count);
    Ok(count > 0)
}

#[tauri::command]
pub async fn retrieve_tags(filter: String, kind: Option<TagKind>) -> Result<Vec<Tag>> {
    let keywords = filter.trim().replace("  ", " ").split_whitespace().map(|word| word.into()).collect::<Vec<String>>();
    let tags = if kind.is_none() {
        sqlx::query_as::<_, Tag>(
            r#"
            SELECT * FROM tags
            WHERE name LIKE '%' || $1 || '%'
            ORDER BY parent_id, id
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
            ORDER BY parent_id, id
            "#)
            .bind(keywords.join("%")).bind(kind.unwrap() as u8)
            .fetch_all(db())
            .await?
    };
    info!("Retrieved {} tags.", tags.len());
    Ok(tags)
}

#[tauri::command]
pub async fn frequent_tags(kind: TagKind, limit: Option<u32>) -> Result<Vec<Tag>> {
    let limit = limit.unwrap_or(5).clamp(1, 20) as i64;
    let window: i64 = 50;
    let tags = if kind == TagKind::Activity {
        sqlx::query_as::<_, Tag>(
            r#"
            WITH recent AS (
                SELECT a.tag_id AS tag_id, a.id AS recency
                FROM activities a
                INNER JOIN tags ON tags.id = a.tag_id
                WHERE tags.kind = $1
                ORDER BY a.id DESC
                LIMIT $2
            ),
            ranked AS (
                SELECT tag_id, COUNT(*) AS freq, MAX(recency) AS last_seen
                FROM recent
                GROUP BY tag_id
            )
            SELECT tags.id, tags.name, tags.remark, tags.color, tags.icon, tags.kind, tags.parent_id
            FROM ranked
            INNER JOIN tags ON tags.id = ranked.tag_id
            ORDER BY ranked.freq DESC, ranked.last_seen DESC
            LIMIT $3
            "#)
            .bind(kind as u8)
            .bind(window)
            .bind(limit)
            .fetch_all(db())
            .await?
    } else {
        sqlx::query_as::<_, Tag>(
            r#"
            WITH recent AS (
                SELECT t.tag_id AS tag_id, t.time AS recency, t.id AS recency_id
                FROM transactions t
                INNER JOIN tags ON tags.id = t.tag_id
                WHERE tags.kind = $1
                ORDER BY t.time DESC, t.id DESC
                LIMIT $2
            ),
            ranked AS (
                SELECT tag_id, COUNT(*) AS freq, MAX(recency) AS last_seen, MAX(recency_id) AS last_id
                FROM recent
                GROUP BY tag_id
            )
            SELECT tags.id, tags.name, tags.remark, tags.color, tags.icon, tags.kind, tags.parent_id
            FROM ranked
            INNER JOIN tags ON tags.id = ranked.tag_id
            ORDER BY ranked.freq DESC, ranked.last_seen DESC, ranked.last_id DESC
            LIMIT $3
            "#)
            .bind(kind as u8)
            .bind(window)
            .bind(limit)
            .fetch_all(db())
            .await?
    };
    info!("Retrieved {} frequent tags for kind {:?}.", tags.len(), kind);
    Ok(tags)
}

#[tauri::command]
pub async fn delete_tag(id: u32) -> Result<()> {
    debug!("Deleting tag(id = {})", id);
    let tag = super::service::get_tag_by_id(id).await?;
    let mut tx = db().begin().await?;

    if tag.kind == TagKind::Activity {
        let activity_count = activity::service::count_activities_with_tag(id).await?;
        if activity_count > 0 {
            return Err(Error::InvalidParameter(format!(
                "cannot delete tag(id = {}): {activity_count} activity(ies) still use it",
                id
            )));
        }
    } else {
        // delete all related transactions first, including transactions related to child tags
        let transaction_ids = sqlx::query(
            r#"
            SELECT id FROM transactions
            WHERE tag_id in (
                WITH RECURSIVE tag_tree(id) AS (
                    SELECT id FROM tags WHERE id = $1
                    UNION ALL
                    SELECT t.id FROM tags t JOIN tag_tree tt ON t.parent_id = tt.id
                )
                SELECT id FROM tag_tree
            )"#)
            .bind(id)
            .fetch_all(&mut *tx)
            .await?
            .into_iter()
            .map(|row| row.get("id"))
            .collect::<Vec<u32>>();
        debug!("{} transactions to be deleted", transaction_ids.len());
        for id in transaction_ids {
            crate::transaction::service::delete_transaction(&mut *tx, id).await?;
        }
    }

    // delete self directly and all child tags will be deleted automatically(ON DELETE CASCADE)
    sqlx::query(
        r#"
        DELETE FROM tags
        WHERE id = $1
        "#)
        .bind(id)
        .execute(&mut *tx)
        .await?;

    // commit the transaction
    tx.commit().await?;
    info!("Tag(id = {}) deleted", id);
    Ok(())
}