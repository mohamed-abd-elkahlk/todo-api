use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}
#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
}
