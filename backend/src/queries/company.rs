use types::company::{Company, DbCompany, NewCompany, UpdatedCompany};

use super::prelude::*;

pub async fn create_company(company: &NewCompany) -> ApiResult<Company> {
    let query = format!(
        "INSERT INTO {} (id, name, picture, bio) VALUES ($1, $2, $3, $4) RETURNING *",
        String::from(TableNames::Company)
    );

    let company = sqlx::query_as::<_, DbCompany>(&query)
        .bind(ulid())
        .bind(&company.name)
        .bind(&company.picture)
        .bind(&company.bio)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(company)
}

pub async fn get_company_by_id(id: &str) -> ApiResult<Company> {
    let query = format!(
        "SELECT * FROM {} WHERE id = $1",
        String::from(TableNames::Company)
    );

    let company = sqlx::query_as::<_, DbCompany>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(company)
}

pub async fn get_all_companies() -> ApiResult<Vec<Company>> {
    let query = format!("SELECT * FROM {}", String::from(TableNames::Company));

    let companies = sqlx::query_as::<_, DbCompany>(&query)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|company| company.into())
        .collect();

    Ok(companies)
}

pub async fn update_company(id: &str, company: &UpdatedCompany) -> ApiResult<Company> {
    let query = format!(
        "UPDATE {} SET name = $2, picture = $3, bio = $4 WHERE id = $1 RETURNING *",
        String::from(TableNames::Company)
    );

    let company = sqlx::query_as::<_, DbCompany>(&query)
        .bind(id)
        .bind(&company.name)
        .bind(&company.picture)
        .bind(&company.bio)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(company)
}

pub async fn delete_company(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1 RETURNING *",
        String::from(TableNames::Company)
    );

    sqlx::query_as::<_, DbCompany>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?;

    Ok(())
}
