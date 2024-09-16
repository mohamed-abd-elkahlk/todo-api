use rocket::serde::json::Json;
use sqlx::SqlitePool;

use crate::{
    guards::AuthenticatedUser,
    models::todo::{NewTodo, Todo},
};

#[post("/todo", format = "json", data = "<new_todo>")]
pub async fn create_todo(
    db_pool: &rocket::State<SqlitePool>,
    user: AuthenticatedUser,
    new_todo: Json<NewTodo>,
) -> Json<Todo> {
    let user_id: i64 = user.user_id.parse().expect("error");
    let result = sqlx::query!(
        "INSERT INTO todos (title, completed,user_id) VALUES (?, ?,?)",
        new_todo.title,
        false,
        user.user_id
    )
    .execute(db_pool.inner())
    .await
    .unwrap();

    let todo = Todo {
        id: result.last_insert_rowid(),
        title: new_todo.title.clone(),
        completed: false,
        user_id,
    };
    Json(todo)
}

#[get("/todo")]
pub async fn get_all_todos(
    db_pool: &rocket::State<SqlitePool>,
    user: AuthenticatedUser,
) -> Json<Vec<Todo>> {
    let todos = sqlx::query!(
        "SELECT id, user_id, title, completed FROM todos WHERE user_id = ?",
        user.user_id
    )
    .fetch_all(db_pool.inner())
    .await
    .unwrap();

    let todos: Vec<Todo> = todos
        .into_iter()
        .map(|row| Todo {
            id: row.id,
            user_id: row.user_id,
            title: row.title,
            completed: match row.completed {
                1 => true,
                0 => false,
                _ => true,
            },
        })
        .collect();

    Json(todos)
}

#[delete("/todo/<id>")]
pub async fn delete_todo(
    db_pool: &rocket::State<SqlitePool>,
    user: AuthenticatedUser,
    id: i64,
) -> &'static str {
    sqlx::query!(
        "DELETE FROM todos WHERE id = ? And user_id = ? ",
        id,
        user.user_id
    )
    .execute(db_pool.inner())
    .await
    .unwrap();
    "Todo deleted!"
}

#[put("/todo/<id>")]
pub async fn complete_todo(
    db_pool: &rocket::State<SqlitePool>,
    user: AuthenticatedUser,
    id: i64,
) -> &'static str {
    sqlx::query!(
        "UPDATE todos SET completed = true WHERE id = ? AND user_id = ?",
        id,
        user.user_id
    )
    .execute(db_pool.inner())
    .await
    .unwrap();
    "Todo completed!"
}

#[get("/todo/<id>")]
pub async fn get_todo(
    db_pool: &rocket::State<SqlitePool>,
    user: AuthenticatedUser,
    id: i64,
) -> Json<Todo> {
    let user_id = user.user_id.parse().expect("error while pares the id");
    let record = sqlx::query!(
        "SELECT id, title, completed FROM todos WHERE id = ? AND user_id = ?",
        id,
        user.user_id
    )
    .fetch_one(db_pool.inner())
    .await
    .unwrap();
    let todo = Todo {
        id: record.id,
        title: record.title,
        completed: record.completed != 0,
        user_id,
    };
    Json(todo)
}
