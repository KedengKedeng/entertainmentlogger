use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct FranchiseToUserLink {
    pub id: String,
    pub date_added: chrono::DateTime<chrono::Utc>,

    pub user_id: String,
    pub franchise_id: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbFranchiseToUserLink {
    pub id: String,
    pub date_added: chrono::DateTime<chrono::Utc>,

    pub user_id: String,
    pub franchise_id: String
}

impl From<DbFranchiseToUserLink> for FranchiseToUserLink {
    fn from(db_franchise_to_user_link: DbFranchiseToUserLink) -> Self {
        Self {
            id: db_franchise_to_user_link.id,
            date_added: db_franchise_to_user_link.date_added,

            user_id: db_franchise_to_user_link.user_id,
            franchise_id: db_franchise_to_user_link.franchise_id
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewFranchiseToUserLink {
    pub user_id: String,
    pub franchise_id: String
}