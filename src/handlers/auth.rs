use std::env;

use crate::models::user::User;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
};
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

#[post("/login", data = "<user>")]
pub async fn login(
    db_pool: &rocket::State<SqlitePool>,
    cookies: &CookieJar<'_>,
    user: Json<User>,
) -> Result<Json<User>, String> {
    let email = &user.email;

    // Check if user exists
    let result = sqlx::query!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE email = ?) as user_exists",
        email
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| "Failed to execute query")?;

    if result.user_exists != 1 {
        return Err("User does not exist".to_string());
    }

    // Get user data
    let record = sqlx::query!(
        "SELECT id, email, username, password FROM users WHERE email = ?",
        user.email
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| "Failed to retrieve user data")?;

    // Check password hash
    let parsed_hash =
        PasswordHash::new(&record.password).map_err(|_| "Failed to parse password hash")?;
    let password_verification =
        Argon2::default().verify_password(user.password.as_bytes(), &parsed_hash);

    if password_verification.is_err() {
        return Err("Invalid password".to_string());
    }

    // Return the user object without the password
    let response_user = User {
        email: record.email,
        username: record.username,
        id: Some(record.id.expect("ID missing in the database record")),
        password: "".to_string(), // Do not return the password
    };
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not found");
    let token = encode(
        &Header::default(),
        &response_user,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(|_| Status::InternalServerError)
    .unwrap();

    cookies.add_private(Cookie::build(("auth_token", token.clone())).http_only(true));

    Ok(Json(response_user))
}
