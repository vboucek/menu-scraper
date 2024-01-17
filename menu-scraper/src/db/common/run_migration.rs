use std::sync::Arc;

use crate::db::common::error::DbResultSingle;
use sqlx::sqlx_macros::migrate;
use sqlx::PgPool;

#[inline]
/// Run the migration script to ensure the database has been migrated.
/// Fails if the database table does not exist!
///
/// # Errors
/// When the migrations folder could not be found, or database connection cannot be established,
/// or when the code in the migrations is incorrect.
pub async fn run_migration(pool: Arc<PgPool>) -> DbResultSingle<()> {
    migrate!("./migrations").run(&*pool).await?;

    Ok(())
}
