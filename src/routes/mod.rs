use crate::handlers::{
    auth::{login, register_user},
    todo::{complete_todo, create_todo, delete_todo, get_all_todos, get_todo},
};
use rocket::Route;

pub fn get_todo_routes() -> Vec<Route> {
    routes![
        get_todo,
        create_todo,
        complete_todo,
        delete_todo,
        get_all_todos
    ]
}

pub fn get_auth_routes() -> Vec<Route> {
    routes![register_user, login]
}
