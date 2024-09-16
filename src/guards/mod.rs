use std::env;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (typically the user ID)
    exp: usize,  // Expiration time (as a Unix timestamp)
}

pub struct AuthenticatedUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Get the "Authorization" cookies
        if let Some(auth_cookies) = request.cookies().get_private("auth_token") {
            if let Some(token) = auth_cookies.value_raw() {
                // Verify and decode the token
                if let Ok(claims) = verify_token(token) {
                    // If the token is valid, return an AuthenticatedUser instance
                    return Outcome::Success(AuthenticatedUser {
                        user_id: claims.sub,
                    });
                }
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
    let validation = Validation::new(Algorithm::HS256); // Ensure you match the algorithm used for encoding

    // Decode the token
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims) // Return the claims if valid
}
