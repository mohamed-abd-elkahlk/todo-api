use std::{env, fs::OpenOptions};

use sqlx::{Pool, Sqlite, SqlitePool};

pub async fn setup_db() -> Pool<Sqlite> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let path = "db.sqlite";
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open or create file");
    let db_pool = SqlitePool::connect(&database_url).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0
        )",
    )
    .execute(&db_pool)
    .await
    .unwrap();
    db_pool
}
