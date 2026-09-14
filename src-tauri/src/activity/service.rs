use tracing::{debug, info};

use crate::sql::db;
use crate::Result;

use super::Activity;

pub async fn get_activity_by_id(id: u32) -> Result<Activity> {
    debug!("Getting activity with id: {}", id);
    let activity = sqlx::query_as::<_, Activity>(
        r#"
        SELECT id, name, remark, color, icon, open, tag_id
        FROM activities
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(db())
    .await?;
    info!("Activity(id = {}) found", activity.id);
    Ok(activity)
}

pub async fn count_activities_with_tag(tag_id: u32) -> Result<i64> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) FROM activities
        WHERE tag_id IN (
            WITH RECURSIVE tag_tree(id) AS (
                SELECT id FROM tags WHERE id = $1
                UNION ALL
                SELECT t.id FROM tags t JOIN tag_tree tt ON t.parent_id = tt.id
            )
            SELECT id FROM tag_tree
        )
        "#,
    )
    .bind(tag_id)
    .fetch_one(db())
    .await?;
    debug!("Activities using tag tree of {}: {}", tag_id, count);
    Ok(count)
}
