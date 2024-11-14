use chrono::{ Days, Duration };
use jsonwebtoken::{ Algorithm, EncodingKey };
use poem::web::cookie::Cookie;
use argon2::{ password_hash::{ rand_core::OsRng, SaltString }, Argon2, PasswordHasher };
use sqlx::types::time::OffsetDateTime;

use crate::{
    error::{ApiError, ApiResult},
    jwt_middleware::UserSession,
    types::user::User ,
    CONFIG,
};

pub fn ulid() -> String {
    ulid::Ulid::new().to_string()
}

pub fn create_jwt(user: &User) -> ApiResult<Cookie> {
    let header = jsonwebtoken::Header::new(Algorithm::HS512);

    let claims = UserSession {
        email: user.email.to_owned(),
        id: user.id.to_owned(),
        username: user.username.to_owned(),
        exp: OffsetDateTime::now_utc().unix_timestamp() + Duration::days(69).num_seconds(),
    };

    let jwt = jsonwebtoken
        ::encode(&header, &claims, &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()))
        .map_err(ApiError::JwtError)?;
    let mut cookie = Cookie::new_with_str("token", jwt);
    cookie.set_expires(chrono::Utc::now().checked_add_days(Days::new(69)).unwrap());
    cookie.set_secure(false);
    cookie.set_path("/");
    Ok(cookie)
}

pub fn hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).expect("Failed to hash password").to_string();

    password_hash
}