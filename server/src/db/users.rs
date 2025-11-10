use sqlx::{SqlitePool, Row};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

/// Create a new user
pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    password_hash: &str,
    email: Option<&str>,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash, email) VALUES (?1, ?2, ?3)"
    )
    .bind(username)
    .bind(password_hash)
    .bind(email)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Find user by username
pub async fn find_by_username(
    pool: &SqlitePool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, username, password_hash, email, created_at, last_login FROM users WHERE username = ?1"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.get(0),
        username: r.get(1),
        password_hash: r.get(2),
        email: r.get(3),
        created_at: r.get(4),
        last_login: r.get(5),
    }))
}

/// Find user by ID
pub async fn find_by_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, username, password_hash, email, created_at, last_login FROM users WHERE id = ?1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| User {
        id: r.get(0),
        username: r.get(1),
        password_hash: r.get(2),
        email: r.get(3),
        created_at: r.get(4),
        last_login: r.get(5),
    }))
}

/// Update last login timestamp
pub async fn update_last_login(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query("UPDATE users SET last_login = ?1 WHERE id = ?2")
        .bind(now)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Check if username exists
pub async fn username_exists(
    pool: &SqlitePool,
    username: &str,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*) as count FROM users WHERE username = ?1")
        .bind(username)
        .fetch_one(pool)
        .await?;

    let count: i64 = row.get(0);
    Ok(count > 0)
}
