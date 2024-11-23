pub mod user;
pub mod media;
pub mod person;
pub mod franchise;
pub mod user_login;

pub mod prelude {
    pub use crate::error::ApiRespResult as ApiResult;
    pub use crate::queries;
    pub use crate::types;
    pub use poem_openapi::{param::Path, payload::Json, OpenApi};
}
