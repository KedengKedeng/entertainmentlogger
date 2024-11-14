pub mod media_to_user_link;
pub mod franchise_to_user_link;
pub mod franchise_to_media_link;
pub mod genre_to_media_link;
pub mod person_to_media_link;
pub mod person_to_company_link;
pub mod company_to_media_link;

pub(super) mod prelude {
    pub use poem_openapi::Object;
    pub use serde::{Deserialize, Serialize};
    pub use sqlx::FromRow;
}