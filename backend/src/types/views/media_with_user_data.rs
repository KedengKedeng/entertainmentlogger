use crate::types::{media::MediaTypes, relations::media_to_user_link::StatusTypes};

use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct MediaWithUserData {
    pub media_id: String,

    pub release_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,

    pub media_type: MediaTypes,
    pub picture: Option<String>,
    pub name: String,
    pub bio: String,

    pub scene_count: i32,
    pub act_count: i32,

    pub link_id: String,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_started: Option<chrono::DateTime<chrono::Utc>>,
    pub date_ended: Option<chrono::DateTime<chrono::Utc>>,
    pub status_type: StatusTypes,

    pub rating: Option<i8>,

    pub scenes_seen: i32,
    pub acts_seen: i32,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbMediaWithUserData {
    pub media_id: String,

    pub release_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,

    pub media_type: i8,
    pub picture: Option<String>,
    pub name: String,
    pub bio: String,

    pub scene_count: i32,
    pub act_count: i32,

    pub link_id: String,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub date_started: Option<chrono::DateTime<chrono::Utc>>,
    pub date_ended: Option<chrono::DateTime<chrono::Utc>>,
    pub status_type: i8,

    pub rating: Option<i8>,

    pub scenes_seen: i32,
    pub acts_seen: i32,
}

impl From<DbMediaWithUserData> for MediaWithUserData {
    fn from(db_media_with_user_data: DbMediaWithUserData) -> Self {
        Self {
            media_id: db_media_with_user_data.media_id,
            release_date: db_media_with_user_data.release_date,
            end_date: db_media_with_user_data.end_date,

            media_type: db_media_with_user_data.media_type.into(),

            picture: db_media_with_user_data.picture,
            name: db_media_with_user_data.name,
            bio: db_media_with_user_data.bio,            

            scene_count: db_media_with_user_data.scene_count,
            act_count: db_media_with_user_data.act_count,

            link_id: db_media_with_user_data.link_id,
            date_added: db_media_with_user_data.date_added,
            date_started: db_media_with_user_data.date_started,
            date_ended: db_media_with_user_data.date_ended,
            status_type: db_media_with_user_data.status_type.into(),

            rating: db_media_with_user_data.rating,

            scenes_seen: db_media_with_user_data.scenes_seen,
            acts_seen: db_media_with_user_data.acts_seen,
        }
    }
}