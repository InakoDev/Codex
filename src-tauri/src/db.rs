use std::path::Path;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
// use sea_orm_migration::MigratorTrait;

pub async fn connect(app_data_dir: &Path) -> Result<DatabaseConnection, DbErr> {
    std::fs::create_dir_all(app_data_dir).map_err(|err| DbErr::Custom(err.to_string()))?;

    let db_path = app_data_dir.join("codex.sqlite");
    let url = format!("sqlite://{}?mode=rwc", db_path.display());

    let options = ConnectOptions::new(url);
    let db = Database::connect(options).await?;

    // TODO: Migrator::up(&db, None).await?;

    Ok(db)
}