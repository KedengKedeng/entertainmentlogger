use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Person {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub name: String,
    pub picture: Option<String>,
    pub bio: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbPerson {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub name: String,
    pub picture: Option<String>,
    pub bio: String
}

impl From<DbPerson> for Person {
    fn from(db_person: DbPerson) -> Self {
        Self {
            id: db_person.id,
            created: db_person.created,

            name: db_person.name,
            picture: db_person.picture,
            bio: db_person.bio
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewPerson {
    pub name: String,
    pub picture: Option<String>,
    pub bio: String
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct UpdatedPerson {
    pub name: String,
    pub picture: Option<String>,
    pub bio: String
}