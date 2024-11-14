use poem_openapi::Enum;

use super::prelude::*;

#[derive(Serialize, Deserialize, Enum, Debug)]
pub enum StatusTypes {
    PLANNED,
    ONGOING,
    PAUSED,
    DROPPED,
    DONE,
}

impl From<&i8> for StatusTypes {
    fn from(value: &i8) -> Self {
        match value {
            1 => Self::ONGOING,
            2 => Self::PAUSED,
            3 => Self::DROPPED,
            4 => Self::DONE,
            _ => Self::PLANNED,
        }
    }
}

impl From<&StatusTypes> for i8 {
    fn from(value: &StatusTypes) -> Self {
        match value {
            StatusTypes::ONGOING => 1,
            StatusTypes::PAUSED => 2,
            StatusTypes::DROPPED => 3,
            StatusTypes::DONE => 4,
            _ => 0,
        }
    }
}

impl From<i8> for StatusTypes {
    fn from(value: i8) -> Self {
        Self::from(&value)
    }
}

impl From<StatusTypes> for i8 {
    fn from(value: StatusTypes) -> Self {
        Self::from(&value)
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct MediaToUserLink {
    pub id: String,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_started: Option<chrono::DateTime<chrono::Utc>>,
    pub date_ended: Option<chrono::DateTime<chrono::Utc>>,
    pub status_type: StatusTypes,

    pub rating: Option<i8>,

    pub scenes_seen: i32,
    pub acts_seen: i32,

    pub media_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbMediaToUserLink {
    pub id: String,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_started: Option<chrono::DateTime<chrono::Utc>>,
    pub date_ended: Option<chrono::DateTime<chrono::Utc>>,
    pub status_type: i8,

    pub rating: Option<i8>,

    pub scenes_seen: i32,
    pub acts_seen: i32,

    pub media_id: String,
    pub user_id: String,
}

impl From<DbMediaToUserLink> for MediaToUserLink {
    fn from(db_media_to_user_link: DbMediaToUserLink) -> Self {
        Self {
            id: db_media_to_user_link.id,
            date_added: db_media_to_user_link.date_added,
            date_started: db_media_to_user_link.date_started,
            date_ended: db_media_to_user_link.date_ended,
            status_type: i8::into(db_media_to_user_link.status_type),

            rating: db_media_to_user_link.rating,

            scenes_seen: db_media_to_user_link.scenes_seen,
            acts_seen: db_media_to_user_link.acts_seen,

            media_id: db_media_to_user_link.media_id,
            user_id: db_media_to_user_link.user_id,
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewMediaToUserLink {
    pub media_id: String,
    pub user_id: String,

    pub date_started: Option<chrono::DateTime<chrono::Utc>>,
    pub date_ended: Option<chrono::DateTime<chrono::Utc>>,
    pub status_type: StatusTypes,

    pub rating: Option<i8>,

    pub scenes_seen: i32,
    pub acts_seen: i32,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct UpdatedMediaToUserLink {
    pub date_started: Option<chrono::DateTime<chrono::Utc>>,
    pub date_ended: Option<chrono::DateTime<chrono::Utc>>,
    pub status_type: StatusTypes,

    pub rating: Option<i8>,

    pub scenes_seen: i32,
    pub acts_seen: i32,
}
