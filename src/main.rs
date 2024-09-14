#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    title: String,
    completed: bool,
}

#[launch]
async fn rocket() -> Result<(), rocket::Error> {
    let db_pool = SqlitePool::connect("sqlite://Database.db").await.unwrap();
    sqlx::query(
        "CREATE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOL NOT NULL DEFAULT 0
        )",
    )
    .execute(&db_pool)
    .await
    .unwrap();
    rocket::build().manage(db_pool).launch().await?;
    Ok(())
}
