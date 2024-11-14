pub mod user;
pub mod franchise;
pub mod media;
pub mod genre;
pub mod person;
pub mod company;
pub mod relations;
pub mod views;
pub mod table_names;

pub(super) mod prelude {
    pub use poem_openapi::Object;
    pub use serde::{Deserialize, Serialize};
    pub use sqlx::FromRow;
}