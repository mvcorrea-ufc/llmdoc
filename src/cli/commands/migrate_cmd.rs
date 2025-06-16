// llmdoc/src/cli/commands/migrate_cmd.rs

use std::path::PathBuf;
use std::sync::Arc;

use crate::core::errors::Result;
use crate::core::database::DbConnection;
use crate::migration::MarkdownMigrator;

pub async fn migrate(
    db: Arc<DbConnection>,
    docs_dir: PathBuf,
    dry_run: bool,
) -> Result<()> {
    let mut migrator = MarkdownMigrator::new(db, docs_dir, dry_run);
    migrator.migrate().await?;
    Ok(())
}