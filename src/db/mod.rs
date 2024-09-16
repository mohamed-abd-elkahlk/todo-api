use sqlx::{Pool, Sqlite, SqlitePool};
use std::env;

pub async fn setup_db() -> Pool<Sqlite> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&db_pool)
    .await
    .expect("Failed to create users table");

    // Create the todos table if it doesn't exist, linked to users
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0,
            user_id INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
    )
    .execute(&db_pool)
    .await
    .expect("Failed to create todos table");

    db_pool
}
