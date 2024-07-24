use crate::model::models::Claims;
use dotenv::dotenv;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;

pub async fn generate_jwt(user_id: &str) -> Result<String, Error> {
    dotenv().ok();

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
}
