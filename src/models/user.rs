use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
}
