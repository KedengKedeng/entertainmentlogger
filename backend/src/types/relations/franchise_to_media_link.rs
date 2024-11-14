use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct FranchiseToMediaLink {
    pub id: String,

    pub franchise_id: String,
    pub media_id: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbFranchiseToMediaLink {
    pub id: String,

    pub franchise_id: String,
    pub media_id: String
}

impl From<DbFranchiseToMediaLink> for FranchiseToMediaLink {
    fn from(db_franchise_to_media_link: DbFranchiseToMediaLink) -> Self {
        Self {
            id: db_franchise_to_media_link.id,

            franchise_id: db_franchise_to_media_link.franchise_id,
            media_id: db_franchise_to_media_link.media_id
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewFranchiseToMediaLink {
    pub media_id: String,
    pub franchise_id: String
}