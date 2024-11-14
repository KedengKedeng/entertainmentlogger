use super::prelude::*;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct PersonToCompanyLink {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub from: chrono::DateTime<chrono::Utc>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,

    pub person_id: String,
    pub company_id: String
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DbPersonToCompanyLink {
    pub id: String,
    pub created: chrono::DateTime<chrono::Utc>,

    pub from: chrono::DateTime<chrono::Utc>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,

    pub person_id: String,
    pub company_id: String
}

impl From<DbPersonToCompanyLink> for PersonToCompanyLink {
    fn from(db_person_to_company_link: DbPersonToCompanyLink) -> Self {
        Self {
            id: db_person_to_company_link.id,
            created: db_person_to_company_link.created,

            from: db_person_to_company_link.from,
            to: db_person_to_company_link.to,

            person_id: db_person_to_company_link.person_id,
            company_id: db_person_to_company_link.company_id
        }
    }
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct NewPersonToCompanyLink {
    pub person_id: String,
    pub company_id: String,
    pub from: chrono::DateTime<chrono::Utc>,
    pub to: Option<chrono::DateTime<chrono::Utc>>
}

pub struct UpdatedPersonToCompanyLink {
    pub from: chrono::DateTime<chrono::Utc>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
}