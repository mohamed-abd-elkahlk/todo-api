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

#[derive(Deserialize, FromForm)]
pub struct Pagination {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
// Struct to return paginated data and metadata
#[derive(Serialize)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub total_pages: usize,
    pub total_items: usize,
    pub current_page: usize,
    pub page_size: usize,
}
