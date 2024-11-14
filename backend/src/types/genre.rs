use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Genre {
    pub created: chrono::DateTime<chrono::Utc>,

    pub name: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbGenre {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub name: String
}

impl From<DbGenre> for Genre {
    fn from(db_genre: DbGenre) -> Self {
        Self {
            created: db_genre.created,

            name: db_genre.name
        }
    }
}

pub struct NewGenre {
    pub name: String
}

pub struct UpdatedGenre {
    pub name: String
}