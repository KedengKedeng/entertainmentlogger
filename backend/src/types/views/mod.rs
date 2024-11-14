pub mod media_with_user_data;

pub(super) mod prelude {
    pub use poem_openapi::Object;
    pub use serde::{Deserialize, Serialize};
    pub use sqlx::FromRow;
}