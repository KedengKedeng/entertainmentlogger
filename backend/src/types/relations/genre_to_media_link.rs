use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct GenreToMediaLink {
    pub id: String,

    pub genre_id: String,
    pub media_id: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbGenreToMediaLink {
    pub id: String,

    pub genre_id: String,
    pub media_id: String,
}

impl From<DbGenreToMediaLink> for GenreToMediaLink {
    fn from(db_genre_to_media_link: DbGenreToMediaLink) -> Self {
        Self {
            id: db_genre_to_media_link.id,
            
            genre_id: db_genre_to_media_link.genre_id,
            media_id: db_genre_to_media_link.media_id,
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewGenreToMediaLink {
    pub genre_id: String,
    pub media_id: String,
}