use std::env;

use crate::{guards::Claims, models::user::User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
};
use sqlx::SqlitePool;
use todo_api::{get_current_timestamp, get_expiration_time};

// TODO: creat macro for password hash

#[post("/register", data = "<new_user>")]
pub async fn register_user(
    db_pool: &rocket::State<SqlitePool>,
    new_user: Json<User>,
) -> Result<Json<User>, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_bytes = Some(&new_user.password)
        .unwrap()
        .as_deref()
        .map(|p| p.as_bytes())
        .unwrap_or(&b""[..]); // Default
    let password_hash = match argon2.hash_password(password_bytes, &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return Err("Error hashing the password".to_string()),
    };

    // Insert the new user into the database
    let query_result = sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
        new_user.username,
        new_user.email,
        password_hash
    )
    .execute(db_pool.inner())
    .await
    .unwrap();

    // Return the user with the newly inserted ID
    let user = User {
        id: Some(query_result.last_insert_rowid()),
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password: None, // Optionally omit password from response for security
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
    let password = user
        .password
        .as_deref()
        .map(|f| f.to_string())
        .expect("error while parse the password");
    let password_verification =
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    if password_verification.is_err() {
        return Err("Invalid password".to_string());
    }

    // Return the user object without the password
    let response_user = User {
        email: record.email,
        username: record.username,
        id: Some(record.id.expect("ID missing in the database record")),
        password: None, // Do not return the password
    };
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not found");

    let claims = Claims {
        exp: get_expiration_time(3600), // Implement this function as needed
        sub: response_user
            .id
            .map(|b| b.to_string())
            .expect("error while parse id to string"),
        iat: get_current_timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(|_| Status::InternalServerError)
    .unwrap();
    cookies.add(Cookie::build(("auth_token", token)).http_only(true));

    Ok(Json(response_user))
}
