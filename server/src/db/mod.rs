pub mod users;
pub mod characters;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

/// Initialize database and run migrations
pub async fn init_database(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Create database file if it doesn't exist
    if database_url.starts_with("sqlite://") {
        let path = database_url.strip_prefix("sqlite://").unwrap();
        let db_path = Path::new(path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            if !parent.exists() && parent != Path::new("") {
                log::info!("Creating database directory: {:?}", parent);
                std::fs::create_dir_all(parent)
                    .map_err(|e| sqlx::Error::Io(e))?;
            }
        }
        
        if !db_path.exists() {
            log::info!("Creating new database file: {}", path);
            // Touch the file to create it
            std::fs::File::create(db_path)
                .map_err(|e| sqlx::Error::Io(e))?;
        }
    }

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    log::info!("Database connected successfully");

    // Run migrations manually (simple approach)
    run_migrations(&pool).await?;

    Ok(pool)
}

/// Run database migrations
async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    log::info!("Running database migrations...");

    // Migration 001: Create users table
    sqlx::query(include_str!("../../migrations/001_create_users.sql"))
        .execute(pool)
        .await?;
    log::info!("Migration 001_create_users completed");

    // Migration 002: Create characters table
    sqlx::query(include_str!("../../migrations/002_create_characters.sql"))
        .execute(pool)
        .await?;
    log::info!("Migration 002_create_characters completed");

    log::info!("All migrations completed successfully");
    Ok(())
}
