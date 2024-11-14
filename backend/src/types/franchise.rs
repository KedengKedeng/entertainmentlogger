use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Franchise {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub edited: chrono::DateTime<chrono::Utc>,
    pub first_showing: chrono::DateTime<chrono::Utc>,

    pub picture: Option<String>,
    pub name: String,
    pub bio: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbFranchise {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub edited: chrono::DateTime<chrono::Utc>,
    pub first_showing: chrono::DateTime<chrono::Utc>,

    pub picture: Option<String>,
    pub name: String,
    pub bio: String
}

impl From<DbFranchise> for Franchise {
    fn from(db_franchise: DbFranchise) -> Self {
        Self {
            id: db_franchise.id,
            created: db_franchise.created,
            edited: db_franchise.edited,
            first_showing: db_franchise.first_showing,

            picture: db_franchise.picture,
            name: db_franchise.name,
            bio: db_franchise.bio
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewFranchise {
    pub picture: Option<String>,
    pub name: String,
    pub bio: String
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct UpdatedFranchise {
    pub picture: Option<String>,
    pub name: String,
    pub bio: String
}