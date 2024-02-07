use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use super::authenticator;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}
pub struct JwtAuthenticator {}

#[async_trait::async_trait]
impl authenticator::Authenticator for JwtAuthenticator {
    async fn authenticate(&self, token: &str) -> Result<(), &'static str> {
        if token == "valid_token" {
            Ok(())
        } else {
            Err("Invalid token")
        }
    }
}

impl JwtAuthenticator {
    pub fn validate_token(secret: &str, token: &str) -> Result<(), &'static str> {
        let key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        decode::<Claims>(token, &key, &validation)
            .map_err(|error| match error.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken
                | jsonwebtoken::errors::ErrorKind::InvalidSignature
                | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
                }
                _ => {
                    eprintln!("Error validating token: {:?}", error);
                    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
                }
            })
            .map(|_claim| true)
    }
}