use crate::models::user::User;
use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String, //subject - represents user tj. his id
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub exp: i64,
    pub iat: i64, //issued at time
}

pub fn generate(user: &User) -> String {
    let secret_key = match dotenv::var("JWT_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => "".to_string(),
    };

    let jwt_duration = match dotenv::var("JWT_LIFETIME_IN_SECONDS") {
        Ok(duration) => duration,
        Err(_) => "300".to_string(),
    };

    let jwt_duration: i64 = jwt_duration.parse().unwrap_or(300);
    let exp = Utc::now() + chrono::Duration::seconds(jwt_duration);

    let claims = Claims {
        sub: String::from(&user.id),
        first_name: String::from(&user.first_name),
        last_name: String::from(&user.last_name),
        email: String::from(&user.email),
        pass: String::from(&user.pass),
        is_admin: user.is_admin,
        created_at: user.created_at,
        updated_at: user.updated_at,
        exp: exp.timestamp(),
        iat: Utc::now().timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .unwrap_or_default()
}

pub fn verify_token(token: String) -> Result<User, jsonwebtoken::errors::Error> {
    let secret_key = match dotenv::var("JWT_SECRET_KEY") {
        Ok(key) => key,
        Err(_) => "".to_string(),
    };

    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret_key.as_bytes()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    )?;

    Ok(User::create_user_from_jwt(&token_data.claims))
}
