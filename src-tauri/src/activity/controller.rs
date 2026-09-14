use tracing::{debug, info};

use crate::sql::db;
use crate::summary::SummaryType;
use crate::tag::{self, TagKind};
use crate::{Error, Result};

use super::dto::{ActivityBalanceDto, SummaryByActivityDto, SummaryByActivityTagDto};
use super::vo::UpdateActivityVo;
use super::{Activity, ActivityDetail};

async fn require_activity_tag(tag_id: u32) -> Result<tag::Tag> {
    let tag = tag::service::get_tag_by_id(tag_id).await?;
    if tag.kind != TagKind::Activity {
        return Err(Error::InvalidTagType {
            given: tag.kind,
            allow: vec![TagKind::Activity],
        });
    }
    Ok(tag)
}

#[tauri::command]
pub async fn create_activity(name: String, remark: String, color: String, icon: String, tag_id: u32) -> Result<()> {
    debug!("Creating activity: {} tag_id={}", name, tag_id);
    require_activity_tag(tag_id).await?;
    sqlx::query(
        r#"
        INSERT INTO activities (name, remark, color, icon, open, tag_id)
        VALUES ($1, $2, $3, $4, 1, $5)
        "#,
    )
    .bind(name)
    .bind(remark)
    .bind(color)
    .bind(icon)
    .bind(tag_id)
    .execute(db())
    .await?;
    info!("Activity created");
    Ok(())
}

#[tauri::command]
pub async fn update_activity(vo: UpdateActivityVo) -> Result<()> {
    debug!("Updating activity(id = {})", vo.id);
    require_activity_tag(vo.tag_id).await?;
    sqlx::query(
        r#"
        UPDATE activities
        SET name = $1, remark = $2, color = $3, icon = $4, open = $5, tag_id = $6
        WHERE id = $7
        "#,
    )
    .bind(vo.name)
    .bind(vo.remark)
    .bind(vo.color)
    .bind(vo.icon)
    .bind(vo.open)
    .bind(vo.tag_id)
    .bind(vo.id)
    .execute(db())
    .await?;
    info!("Activity(id = {}) updated", vo.id);
    Ok(())
}

#[tauri::command]
pub async fn set_activity_open(id: u32, open: bool) -> Result<()> {
    debug!("Setting activity(id = {}) open = {}", id, open);
    sqlx::query(
        r#"
        UPDATE activities
        SET open = $1
        WHERE id = $2
        "#,
    )
    .bind(open)
    .bind(id)
    .execute(db())
    .await?;
    info!("Activity(id = {}) open set to {}", id, open);
    Ok(())
}

#[tauri::command]
pub async fn get_activity_by_id(id: u32) -> Result<ActivityDetail> {
    let activity = super::service::get_activity_by_id(id).await?;
    let tag = tag::service::get_tag_by_id(activity.tag_id).await?;
    Ok(ActivityDetail { activity, tag })
}

#[tauri::command]
pub async fn retrieve_activities(open_only: Option<bool>) -> Result<Vec<Activity>> {
    debug!("Retrieving activities open_only={:?}", open_only);
    let activities = if open_only == Some(true) {
        sqlx::query_as::<_, Activity>(
            r#"
            SELECT id, name, remark, color, icon, open, tag_id
            FROM activities
            WHERE open = 1
            ORDER BY id
            "#,
        )
        .fetch_all(db())
        .await?
    } else {
        sqlx::query_as::<_, Activity>(
            r#"
            SELECT id, name, remark, color, icon, open, tag_id
            FROM activities
            ORDER BY open DESC, id
            "#,
        )
        .fetch_all(db())
        .await?
    };
    info!("Retrieved {} activities.", activities.len());
    Ok(activities)
}

#[tauri::command]
pub async fn get_activity_balances(currency_code: String, open_only: Option<bool>) -> Result<Vec<ActivityBalanceDto>> {
    debug!("Getting activity balances currency={} open_only={:?}", currency_code, open_only);
    let balances = if open_only == Some(true) {
        sqlx::query_as::<_, ActivityBalanceDto>(
            r#"
            SELECT
                activities.id,
                COALESCE(SUM(CASE WHEN tags.kind = $1 THEN COALESCE(ts.expense, transactions.amount) ELSE 0 END), 0) AS expense,
                COALESCE(SUM(CASE WHEN tags.kind = $2 THEN transactions.amount ELSE 0 END), 0) AS income
            FROM activities
            LEFT JOIN transactions ON transactions.activity_id = activities.id
            LEFT JOIN transaction_splits ts ON transactions.split_id = ts.id
            LEFT JOIN tags ON transactions.tag_id = tags.id
            LEFT JOIN wallets ON transactions.wallet_id = wallets.id
            WHERE activities.open = 1 AND (wallets.id IS NULL OR wallets.currency_code = $3)
            GROUP BY activities.id
            "#,
        )
        .bind(TagKind::Expense as u8)
        .bind(TagKind::Income as u8)
        .bind(&currency_code)
        .fetch_all(db())
        .await?
    } else {
        sqlx::query_as::<_, ActivityBalanceDto>(
            r#"
            SELECT
                activities.id,
                COALESCE(SUM(CASE WHEN tags.kind = $1 THEN COALESCE(ts.expense, transactions.amount) ELSE 0 END), 0) AS expense,
                COALESCE(SUM(CASE WHEN tags.kind = $2 THEN transactions.amount ELSE 0 END), 0) AS income
            FROM activities
            LEFT JOIN transactions ON transactions.activity_id = activities.id
            LEFT JOIN transaction_splits ts ON transactions.split_id = ts.id
            LEFT JOIN tags ON transactions.tag_id = tags.id
            LEFT JOIN wallets ON transactions.wallet_id = wallets.id
            WHERE wallets.id IS NULL OR wallets.currency_code = $3
            GROUP BY activities.id
            "#,
        )
        .bind(TagKind::Expense as u8)
        .bind(TagKind::Income as u8)
        .bind(&currency_code)
        .fetch_all(db())
        .await?
    };
    info!("Retrieved {} activity balances.", balances.len());
    Ok(balances)
}

#[tauri::command]
pub async fn delete_activity(id: u32) -> Result<()> {
    debug!("Deleting activity(id = {})", id);
    sqlx::query(
        r#"
        DELETE FROM activities
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(db())
    .await?;
    info!("Activity(id = {}) deleted", id);
    Ok(())
}

#[tauri::command]
pub async fn get_expense_summary_by_activity(summary_type: Option<SummaryType>, begin: Option<chrono::NaiveDate>, end: Option<chrono::NaiveDate>) -> Result<Vec<SummaryByActivityDto>> {
    debug!("Getting expense summary by activity");
    let (begin_date, end_date) = range_from_summary(summary_type.as_ref(), begin, end);
    let begin = begin_date.and_hms_opt(0, 0, 0).unwrap();
    let end = end_date.and_hms_opt(23, 59, 59).unwrap();
    let result = sqlx::query_as(
        r#"
        SELECT
            activities.id, activities.name, activities.color, activities.icon, activities.open,
            SUM(CASE
                WHEN tags.kind = $3 THEN COALESCE(transaction_splits.expense, transactions.amount)
                ELSE 0
            END) AS summary,
            wallets.currency_code,
            '' AS period
        FROM transactions
        JOIN activities ON transactions.activity_id = activities.id
        JOIN tags ON transactions.tag_id = tags.id
        JOIN wallets ON transactions.wallet_id = wallets.id
        LEFT JOIN transaction_splits ON transactions.split_id = transaction_splits.id
        WHERE transactions.time BETWEEN $1 AND $2
        GROUP BY activities.id, wallets.currency_code
        HAVING summary > 0
        ORDER BY summary DESC
        "#,
    )
    .bind(begin.format(crate::transaction::DATETIME_FORMAT).to_string())
    .bind(end.format(crate::transaction::DATETIME_FORMAT).to_string())
    .bind(TagKind::Expense as u8)
    .fetch_all(db())
    .await?;
    info!("Retrieved {} activity instance summaries.", result.len());
    Ok(result)
}

#[tauri::command]
pub async fn get_expense_summary_by_activity_tag(summary_type: Option<SummaryType>, begin: Option<chrono::NaiveDate>, end: Option<chrono::NaiveDate>) -> Result<Vec<SummaryByActivityTagDto>> {
    debug!("Getting expense summary by activity tag");
    let (begin_date, end_date) = range_from_summary(summary_type.as_ref(), begin, end);
    let begin = begin_date.and_hms_opt(0, 0, 0).unwrap();
    let end = end_date.and_hms_opt(23, 59, 59).unwrap();
    let result = sqlx::query_as(
        r#"
        WITH RECURSIVE tag_tree(root_id, id) AS (
            SELECT id, id FROM tags WHERE parent_id IS NULL AND kind = $4
            UNION ALL
            SELECT tag_tree.root_id, tags.id
            FROM tag_tree JOIN tags ON tags.parent_id = tag_tree.id
        )
        SELECT
            SUM(CASE
                WHEN tx_tags.kind = $3 THEN COALESCE(transaction_splits.expense, transactions.amount)
                ELSE 0
            END) AS summary,
            root.id, root.name, root.remark, root.color, root.icon, root.kind, root.parent_id,
            wallets.currency_code,
            '' AS period
        FROM transactions
        JOIN activities ON transactions.activity_id = activities.id
        JOIN tags AS tx_tags ON transactions.tag_id = tx_tags.id
        JOIN tag_tree ON activities.tag_id = tag_tree.id
        JOIN tags AS root ON tag_tree.root_id = root.id
        JOIN wallets ON transactions.wallet_id = wallets.id
        LEFT JOIN transaction_splits ON transactions.split_id = transaction_splits.id
        WHERE transactions.time BETWEEN $1 AND $2
        GROUP BY tag_tree.root_id, wallets.currency_code
        HAVING summary > 0
        ORDER BY summary DESC
        "#,
    )
    .bind(begin.format(crate::transaction::DATETIME_FORMAT).to_string())
    .bind(end.format(crate::transaction::DATETIME_FORMAT).to_string())
    .bind(TagKind::Expense as u8)
    .bind(TagKind::Activity as u8)
    .fetch_all(db())
    .await?;
    info!("Retrieved {} activity class summaries.", result.len());
    Ok(result)
}

fn range_from_summary(summary_type: Option<&SummaryType>, begin: Option<chrono::NaiveDate>, end: Option<chrono::NaiveDate>) -> (chrono::NaiveDate, chrono::NaiveDate) {
    if let Some(kind) = summary_type {
        let today = chrono::Local::now().naive_local().date();
        let (period_begin, period_end) = kind.get_range_of_date(today);
        (
            std::cmp::max(begin.unwrap_or(period_begin), period_begin),
            std::cmp::min(end.unwrap_or(period_end), period_end),
        )
    } else {
        (
            begin.unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            end.unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()),
        )
    }
}
