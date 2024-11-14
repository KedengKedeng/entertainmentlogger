use types::{
    company::{Company, DbCompany},
    person::{DbPerson, Person},
    relations::person_to_company_link::{
        DbPersonToCompanyLink, NewPersonToCompanyLink, PersonToCompanyLink,
        UpdatedPersonToCompanyLink,
    },
};

use super::prelude::*;

pub async fn create_person_to_company_link(
    new_link: &NewPersonToCompanyLink,
) -> ApiResult<PersonToCompanyLink> {
    let query = format!(
        "INSERT INTO {} (id, person_id, company_id, from, to) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        String::from(TableNames::PersonToCompanyLink)
    );

    let person_to_company_link = sqlx::query_as::<_, DbPersonToCompanyLink>(&query)
        .bind(ulid())
        .bind(&new_link.person_id)
        .bind(&new_link.company_id)
        .bind(new_link.from)
        .bind(new_link.to)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(person_to_company_link)
}

pub async fn get_person_to_company_link(id: &str) -> ApiResult<PersonToCompanyLink> {
    let query = format!(
        "SELECT * FROM {} WHERE id = $1",
        String::from(TableNames::PersonToCompanyLink)
    );

    let person_to_company_link = sqlx::query_as::<_, DbPersonToCompanyLink>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(person_to_company_link)
}

pub async fn get_all_people_from_company(id: &str) -> ApiResult<Vec<Person>> {
    let link_table_name = String::from(TableNames::PersonToCompanyLink);
    let query = format!(
        r#"
        SELECT * FROM {}
        JOIN {} ON people.id = {}.person_id
        WHERE {}.company_id = $1
        GROUP BY people.id
        "#,
        String::from(TableNames::Person),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let people = sqlx::query_as::<_, DbPerson>(&query)
        .bind(id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|person| person.into())
        .collect();

    Ok(people)
}

pub async fn get_all_companies_from_person(person_id: &str) -> ApiResult<Vec<Company>> {
    let link_table_name = String::from(TableNames::PersonToCompanyLink);
    let query = format!(
        r#"
        SELECT companies.* FROM {}
        JOIN {} ON companies.id = {}.company_id
        WHERE {}.person_id = $1
        GROUP BY companies.id
        "#,
        String::from(TableNames::Company),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let companies = sqlx::query_as::<_, DbCompany>(&query)
        .bind(person_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|company| company.into())
        .collect();

    Ok(companies)
}

pub async fn update_person_to_company_link(
    id: &str,
    updated_person_to_company_link: &UpdatedPersonToCompanyLink,
) -> ApiResult<PersonToCompanyLink> {
    let query = format!(
        "UPDATE {} SET from = $1, to = $2 WHERE id = $3 RETURNING *",
        String::from(TableNames::PersonToCompanyLink)
    );

    let person_to_company_link = sqlx::query_as::<_, DbPersonToCompanyLink>(&query)
        .bind(updated_person_to_company_link.from)
        .bind(updated_person_to_company_link.to)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(person_to_company_link)
}

pub async fn delete_person_to_company_link(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1 RETURNING *",
        String::from(TableNames::PersonToCompanyLink)
    );

    sqlx::query_as::<_, DbPersonToCompanyLink>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?;

    Ok(())
}
