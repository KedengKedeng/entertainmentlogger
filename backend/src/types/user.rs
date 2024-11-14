use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct User {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub last_online: chrono::DateTime<chrono::Utc>,

    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,

    pub username: String,
    pub profile_picture: Option<String>,
    pub bio: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbUser {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub last_online: chrono::DateTime<chrono::Utc>,

    pub email: String,
    pub password: String,

    pub username: String,
    pub profile_picture: Option<String>,
    pub bio: String
}

impl From<DbUser> for User {
    fn from(db_user: DbUser) -> Self {
        Self {
            id: db_user.id,
            created: db_user.created,
            last_online: db_user.last_online,

            email: db_user.email,
            password: db_user.password,

            username: db_user.username,
            profile_picture: db_user.profile_picture,
            bio: db_user.bio
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewUser {
    pub email: String,
    pub password: String,

    pub username: String,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct UpdatedUser {
    pub username: String,
    pub profile_picture: Option<String>,
    pub bio: String
}