pub mod media_to_user_link;
pub mod franchise_to_user_link;
pub mod franchise_to_media_link;
pub mod genre_to_media_link;
pub mod person_to_media_link;
pub mod person_to_company_link;

pub (super)mod prelude {
    pub use crate::types;
    pub use types::table_names::TableNames;
    pub use crate::util::ulid;
    pub use crate::error::ApiResult;
    pub use crate::pool;
}