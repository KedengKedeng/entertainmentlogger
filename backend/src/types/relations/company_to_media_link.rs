use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct CompanyToMediaLink {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub role: String,

    pub company_id: String,
    pub media_id: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbCompanyToMediaLink {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub role: String,

    pub company_id: String,
    pub media_id: String
}

impl From<DbCompanyToMediaLink> for CompanyToMediaLink {
    fn from(db_company_to_media_link: DbCompanyToMediaLink) -> Self {
        Self {
            id: db_company_to_media_link.id,
            created: db_company_to_media_link.created,

            role: db_company_to_media_link.role,

            company_id: db_company_to_media_link.company_id,
            media_id: db_company_to_media_link.media_id
        }
    }
}