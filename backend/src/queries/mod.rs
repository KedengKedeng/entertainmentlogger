pub mod user;
pub mod relations;
pub mod franchise;
pub mod media;
pub mod genre;
pub mod person;
pub mod company;

pub (super)mod prelude {
    pub use crate::types;
    pub use types::table_names::TableNames;
    pub use crate::util::ulid;
    pub use crate::error::ApiResult;
    pub use crate::pool;
}