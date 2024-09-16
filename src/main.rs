#[macro_use]
extern crate rocket;
mod db;
mod guards;
mod handlers;
mod models;
mod routes;

use dotenv::dotenv;

use db::setup_db;
use guards::unauthorized;
use rocket::{Build, Rocket};
#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let db_pool = setup_db().await;
    rocket::build()
        .manage(db_pool)
        .register("/", catchers![unauthorized])
        .mount("/", routes::get_todo_routes())
        .mount("/auth", routes::get_auth_routes())
}
