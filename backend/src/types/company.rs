use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Company {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub name: String,
    pub picture: Option<String>,
    pub bio: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbCompany {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub name: String,
    pub picture: Option<String>,
    pub bio: String
}

impl From<DbCompany> for Company {
    fn from(db_company: DbCompany) -> Self {
        Self {
            id: db_company.id,
            created: db_company.created,

            name: db_company.name,
            picture: db_company.picture,
            bio: db_company.bio
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewCompany {
    pub name: String,
    pub picture: Option<String>,
    pub bio: String
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct UpdatedCompany {
    pub name: String,
    pub picture: Option<String>,
    pub bio: String
}