use rocket::{http::Status, response::status, serde::json::Json};
use sqlx::SqlitePool;

use crate::{
    guards::AuthenticatedUser,
    models::todo::{NewTodo, PagedResponse, Pagination, Todo},
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

#[get("/todos?<pagination..>")]
pub async fn get_all_todos(
    db_pool: &rocket::State<SqlitePool>,
    user: AuthenticatedUser,
    pagination: Option<Pagination>,
) -> Json<PagedResponse<Todo>> {
    let page = pagination.as_ref().map_or(1, |p| p.page.unwrap_or(1)) as i64;
    let page_size = pagination
        .as_ref()
        .map_or(10, |p| p.page_size.unwrap_or(10)) as i64;

    let offset = (page - 1) * page_size;
    let user_id = &user.user_id;
    let todos = sqlx::query!(
        "SELECT id, title, user_id, completed FROM todos WHERE user_id = ? LIMIT ? OFFSET ?",
        user_id,
        page_size,
        offset
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
                0 => false,
                _ => true,
            },
        })
        .collect();

    // Fetch total count of todos
    let total: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM todos")
        .fetch_one(db_pool.inner())
        .await
        .unwrap();

    let total_pages = (total as f64 / page_size as f64).ceil() as usize;

    Json(PagedResponse {
        items: todos,
        total_pages,
        total_items: total as usize,
        current_page: page as usize,
        page_size: page_size as usize,
    })
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
) -> Result<status::Custom<&'static str>, status::Custom<&'static str>> {
    let query_result = sqlx::query!(
        "UPDATE todos SET completed = true WHERE id = ? AND user_id = ?",
        id,
        user.user_id
    )
    .execute(db_pool.inner())
    .await;

    match query_result {
        Ok(result) if result.rows_affected() > 0 => {
            Ok(status::Custom(Status::Ok, "Todo completed!"))
        }
        Ok(_) => Err(status::Custom(
            Status::Unauthorized,
            "You are not allowed to perform this action!",
        )),
        Err(e) => {
            eprintln!("Error executing query: {:?}", e);
            Err(status::Custom(
                Status::InternalServerError,
                "Internal server error",
            ))
        }
    }
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
