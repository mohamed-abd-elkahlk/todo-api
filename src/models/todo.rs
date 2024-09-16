use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]

pub struct Todo {
    pub id: i64,
    pub title: String,
    pub user_id: i64,
    pub completed: bool,
}
#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
}
