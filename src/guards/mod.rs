use std::env;

use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (typically the user ID)
    pub exp: usize,  // Expiration time (as a Unix timestamp)
    pub iat: usize,
}
#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Get the "Authorization" cookies
        if let Some(auth_cookie) = request.cookies().get("auth_token") {
            let token = auth_cookie.value();
            // Verify and decode the token
            if let Ok(claims) = verify_token(token) {
                // If the token is valid, return an AuthenticatedUser instance
                return Outcome::Success(AuthenticatedUser {
                    user_id: claims.sub,
                });
            }
        }
        // Return Unauthorized if token verification fails
        Outcome::Error((Status::Unauthorized, ()))
    }
}

// Function to verify and decode JWT token
fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not found");
    // Define the secret key and the validation parameters
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
    // Decode the token
    let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())?;
    Ok(token_data.claims) // Return the claims if valid
}
#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Unauthorized: Invalid token".to_string(),
    })
}
#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "404: Not Found".to_string(),
    })
}
