use crate::models::user::User;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket::serde::json::Json;
use sqlx::{query, SqlitePool};
// TODO: creat macro for password hash
#[post("/register", data = "<new_user>")]
pub async fn register_user(
    db_pool: &rocket::State<SqlitePool>,
    new_user: Json<User>,
) -> Result<Json<User>, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password = new_user.password.as_bytes();
    let password_hash = match argon2.hash_password(password, &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return Err("Error hashing the password".to_string()),
    };

    // Insert the new user into the database
    let query_result = query!(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
        new_user.username,
        new_user.email,
        password_hash
    )
    .execute(db_pool.inner())
    .await;

    // Check for database errors
    if let Err(e) = query_result {
        return Err(format!("Database error: {}", e));
    }
    let last_id = query!("SELECT last_insert_rowid() as id")
        .fetch_one(db_pool.inner())
        .await
        .unwrap()
        .id;

    // Return the user with the newly inserted ID
    let user = User {
        id: Some(last_id),
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password: password_hash, // Optionally omit password from response for security
    };

    Ok(Json(user))
}
