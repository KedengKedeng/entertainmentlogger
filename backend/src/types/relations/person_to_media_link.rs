use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct PersonToMediaLink {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub role: String,

    pub person_id: String,
    pub media_id: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbPersonToMediaLink {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub role: String,

    pub person_id: String,
    pub media_id: String
}

impl From<DbPersonToMediaLink> for PersonToMediaLink {
    fn from(db_person_to_media_link: DbPersonToMediaLink) -> Self {
        Self {
            id: db_person_to_media_link.id,
            created: db_person_to_media_link.created,

            role: db_person_to_media_link.role,

            person_id: db_person_to_media_link.person_id,
            media_id: db_person_to_media_link.media_id
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewPersonToMediaLink {
    pub person_id: String,
    pub media_id: String,
    pub role: String
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct UpdatePersonToMediaLink {
    pub role: String
}