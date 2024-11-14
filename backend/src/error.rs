use poem::error::ParseMultipartError;
use poem_openapi::{ payload::PlainText, ApiResponse };

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Passwords do not match")]
    PasswordsDoNotMatch,
    #[error("Email already registered")]
    EmailAlreadyRegistered,
    #[error("Database Error")] DatabaseError(#[from] sqlx::Error),
    #[error("JWT Error")] JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Oauth2 Error")] Oauth2Error(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Internal server error")] ParseMultipartError(#[from] ParseMultipartError),
    #[error("Bad Request")]
    BadRequest,
    #[error("Password Hash Error")] PasswordHashError(#[from] password_hash::Error),
}
#[allow(unused_mut, unused_variables)]
#[derive(Debug, ApiResponse)]
pub enum ApiErrorResp {
    /// Description 2
    #[oai(status = 409)]
    EmailAlreadyRegistered(PlainText<String>),
    // Not found
    #[oai(status = 404)] NotFound(PlainText<String>),
    // Internal server error
    #[oai(status = 500)] ServerError(PlainText<String>),
    #[oai(status = 400)] BadRequest(PlainText<String>),
    #[oai(status = 413)] PayloadTooLarge(PlainText<String>),
    #[oai(status = 401)] Unauthorized(PlainText<String>),
}

impl From<ApiError> for ApiErrorResp {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::PasswordsDoNotMatch => Self::BadRequest(PlainText(err.to_string())),
            ApiError::EmailAlreadyRegistered =>
                Self::EmailAlreadyRegistered(PlainText(err.to_string())),
            ApiError::DatabaseError(err) =>
                match err {
                    sqlx::Error::RowNotFound => Self::NotFound(PlainText(err.to_string())),
                    _ => Self::ServerError(PlainText(err.to_string())),
                }
            ApiError::JwtError(err) => Self::ServerError(PlainText(err.to_string())),
            ApiError::Oauth2Error(err) => Self::ServerError(PlainText(err)),
            ApiError::Unauthorized => Self::Unauthorized(PlainText(err.to_string())),
            ApiError::NotFound => Self::NotFound(PlainText(err.to_string())),
            ApiError::ParseMultipartError(msg) => Self::BadRequest(PlainText(msg.to_string())),
            ApiError::BadRequest => {
                Self::BadRequest(PlainText(err.to_string()))
            }
            ApiError::PasswordHashError(err) =>
                match err {
                    password_hash::Error::Password =>
                        Self::Unauthorized(PlainText(err.to_string())),
                    _ => Self::ServerError(PlainText(err.to_string())),
                }
        }
    }
}

pub type ApiRespResult<T> = std::result::Result<T, ApiErrorResp>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;