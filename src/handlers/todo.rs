use rocket::serde::json::Json;
use sqlx::SqlitePool;

use crate::models::todo::{NewTodo, Todo};

#[post("/todo", format = "json", data = "<new_todo>")]
pub async fn create_todo(
    db_pool: &rocket::State<SqlitePool>,
    new_todo: Json<NewTodo>,
) -> Json<Todo> {
    let result = sqlx::query!(
        "INSERT INTO todos (title, completed) VALUES (?, ?)",
        new_todo.title,
        false
    )
    .execute(db_pool.inner())
    .await
    .unwrap();

    let todo = Todo {
        id: result.last_insert_rowid(),
        title: new_todo.title.clone(),
        completed: false,
    };
    Json(todo)
}

#[get("/todo")]
pub async fn get_all_todos(db_pool: &rocket::State<SqlitePool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(db_pool.inner())
        .await
        .unwrap();
    Json(todos)
}

#[delete("/todo/<id>")]
pub async fn delete_todo(db_pool: &rocket::State<SqlitePool>, id: i64) -> &'static str {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(db_pool.inner())
        .await
        .unwrap();
    "Todo deleted!"
}

#[put("/todo/<id>")]
pub async fn complete_todo(db_pool: &rocket::State<SqlitePool>, id: i64) -> &'static str {
    sqlx::query!("UPDATE todos SET completed = true WHERE id = ?", id)
        .execute(db_pool.inner())
        .await
        .unwrap();
    "Todo completed!"
}

#[get("/todo/<id>")]
pub async fn get_todo(db_pool: &rocket::State<SqlitePool>, id: i64) -> Json<Todo> {
    let record = sqlx::query!("SELECT id, title, completed FROM todos WHERE id = ?", id)
        .fetch_one(db_pool.inner())
        .await
        .unwrap();
    let todo = Todo {
        id: record.id,
        title: record.title,
        completed: record.completed != 0,
    };
    Json(todo)
}
