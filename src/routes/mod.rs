use crate::{NewTodo, Todo};
use rocket::serde::json::Json;
use sqlx::SqlitePool;

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
pub async fn get_todos(db_pool: &rocket::State<SqlitePool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(db_pool.inner())
        .await
        .unwrap();
    Json(todos)
}
// pub fn delete_todo() {}
// pub fn update_todo() {}
// pub fn get_all_todos() {}
