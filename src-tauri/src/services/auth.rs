use chrono::{Duration, Local};
use diesel::SqliteConnection;
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{enums::ClientRole, models::Client, utils::{verify_hash, ErrorResponse}};

use super::clients::get_client_email;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Claims {
    pub sub: String,
    #[validate(email)]
    pub email: String,
    pub role: ClientRole,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginCredentials {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

impl From<JwtError> for ErrorResponse {
    fn from(value: JwtError) -> Self {
        ErrorResponse {
            error: value.to_string(),
        }
    }
}

impl From<argon2::password_hash::Error> for ErrorResponse {
    fn from(value: argon2::password_hash::Error) -> Self {
        ErrorResponse {
            error: value.to_string(),
        }
    }
}

pub struct Auth {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Auth {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    pub fn generate_token(&self, client: &Client) -> Result<String, ErrorResponse> {
        let now = Local::now();
        let expires_at = now + Duration::hours(24);

        let claims = Claims {
            sub: client.id.clone(),
            email: client.email.clone(),
            role: client.role.clone(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key).map_err(ErrorResponse::from)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, ErrorResponse> {
        let validation = Validation::default();
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(ErrorResponse::from)
    }
}

pub fn login(
    conn: &mut SqliteConnection,
    auth: &Auth,
    credentials: &LoginCredentials,
) -> Result<String, ErrorResponse> {
    let client = get_client_email(conn, credentials.email.as_str()).map_err(|e| ErrorResponse {
        error: "Inalid credentials".to_string(),
    })?;

    let is_valid = verify_hash(&client.password, &credentials.password)?;

    if !is_valid {
        return Err(ErrorResponse {
            error: "Invalid credentials".to_string(),
        });
    }

    auth.generate_token(&client)
}

pub fn verify_auth(token: &str, auth: &Auth) -> Result<Claims, ErrorResponse> {
    auth.verify_token(token)
}
