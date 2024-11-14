use poem_openapi::Enum;

use super::prelude::*;

#[derive(Serialize, Deserialize, Enum, Debug)]
pub enum MediaTypes {
    FILM,
    SERIES,
    BOOK,
    MUSIC
}

impl From<&i8> for MediaTypes {
    fn from(value: &i8) -> Self {
        match value {
            1 => Self::SERIES,
            2 => Self::BOOK,
            3 => Self::MUSIC,
            _ => Self::FILM
        }
    }
}

impl From<&MediaTypes> for i8 {
    fn from(value: &MediaTypes) -> Self {
        match value {
            MediaTypes::SERIES => 1,
            MediaTypes::BOOK => 2,
            MediaTypes::MUSIC => 3,
            _ => 0
        }
    }
}

impl From<i8> for MediaTypes {
    fn from (value: i8) -> Self {
        Self::from(&value)
    }
}

impl From<MediaTypes> for i8 {
    fn from (value: MediaTypes) -> Self {
        Self::from(&value)
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Media {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub edited: chrono::DateTime<chrono::Utc>,

    pub release_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,

    pub media_type: MediaTypes,
    pub picture: Option<String>,
    pub name: String,
    pub bio: String,

    pub scene_count: i32,
    pub act_count: i32
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbMedia {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub edited: chrono::DateTime<chrono::Utc>,

    pub release_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,

    pub media_type: i8,
    pub picture: Option<String>,
    pub name: String,
    pub bio: String,

    pub scene_count: i32,
    pub act_count: i32
}

impl From<DbMedia> for Media {
    fn from(db_media: DbMedia) -> Self {
        Self {
            id: db_media.id,
            created: db_media.created,
            edited: db_media.edited,

            release_date: db_media.release_date,
            end_date: db_media.end_date,

            media_type: db_media.media_type.into(),
            picture: db_media.picture,
            name: db_media.name,
            bio: db_media.bio,

            scene_count: db_media.scene_count,
            act_count: db_media.scene_count
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewMedia {
    pub media_type: MediaTypes,
    pub picture: Option<String>,
    pub name: String,
    pub bio: String,

    pub release_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,

    pub scene_count: i32,
    pub act_count: i32
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct UpdatedMedia {
    pub picture: Option<String>,
    pub name: String,
    pub bio: String,

    pub release_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,

    pub scene_count: i32,
    pub act_count: i32
}