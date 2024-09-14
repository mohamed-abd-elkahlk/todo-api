#[macro_use]
extern crate rocket;
mod db;
mod routes;

use dotenv::dotenv;

use db::setup_db;
use rocket::{
    serde::{Deserialize, Serialize},
    Build, Rocket,
};
use routes::{create_todo, get_todos};
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    title: String,
    completed: bool,
}
#[derive(Deserialize)]
pub struct NewTodo {
    title: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok(); // Load the .env file
    let db_pool = setup_db().await;
    rocket::build()
        .manage(db_pool)
        .mount("/", routes![index, create_todo, get_todos])
}
