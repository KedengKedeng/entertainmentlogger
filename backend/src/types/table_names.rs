pub enum TableNames {
    User,
    MediaToUserLink,
    FranchiseToUserLink,
    Franchise,
    FranchiseToMediaLink,
    Media,
    GenreToMediaLink,
    GenreToFranchiseLink,
    Genre,
    Person,
    PersonToMediaLink,
    PersonToCompanyLink,
    Company,
    CompanyToMediaLink
}

impl From<TableNames> for String {
    fn from(table_name: TableNames) -> Self {
        match table_name {
            TableNames::User => "users".to_string(),
            TableNames::MediaToUserLink => "media_to_user_link".to_string(),
            TableNames::FranchiseToUserLink => "franchise_to_user_link".to_string(),
            TableNames::Franchise => "franchises".to_string(),
            TableNames::FranchiseToMediaLink => "franchise_to_media_link".to_string(),
            TableNames::Media => "media".to_string(),
            TableNames::GenreToMediaLink => "genre_to_media_link".to_string(),
            TableNames::GenreToFranchiseLink => "genre_to_franchise_link".to_string(),
            TableNames::Genre => "genres".to_string(),
            TableNames::Person => "people".to_string(),
            TableNames::PersonToMediaLink => "person_to_media_link".to_string(),
            TableNames::PersonToCompanyLink => "person_to_company_link".to_string(),
            TableNames::Company => "companies".to_string(),
            TableNames::CompanyToMediaLink => "company_to_media_link".to_string()
        }
    }
}