use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};
use poem::{
    handler,
    web::{cookie::CookieJar, Json},
};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    error::{ApiError, ApiRespResult},
    queries::user,
    util::create_jwt,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub success: bool,
}

#[handler]
pub async fn user_login(
    cookie_jar: &CookieJar,
    data: Json<UserLogin>,
) -> ApiRespResult<Json<Response>> {
    let user = user::get_user_by_email(&data.email)
        .await
        .map_err(|_| ApiError::Unauthorized)?;

    let hash = PasswordHash::new(&user.password).map_err(ApiError::from)?;

    Argon2::default()
        .verify_password(data.password.as_bytes(), &hash)
        .map_err(ApiError::from)?;

    cookie_jar.add(create_jwt(&user)?);

    Ok(Json(Response { success: true }))
}
