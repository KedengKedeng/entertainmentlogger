use types::franchise::*;

use super::prelude::*;

pub async fn get_franchise_by_id(id: &str) -> ApiResult<Franchise> {
    let query = format!(
        "SELECT * FROM {} WHERE id = $1",
        String::from(TableNames::Franchise)
    );

    let franchise = sqlx::query_as::<_, DbFranchise>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(franchise)
}

pub async fn create_franchise(new_franchise: &NewFranchise) -> ApiResult<Franchise> {
    let query = format!(
        "INSERT INTO {} (id, name, picture, bio) VALUES ($1, $2, $3, $4) RETURNING *",
        String::from(TableNames::Franchise)
    );

    let franchise = sqlx::query_as::<_, DbFranchise>(&query)
        .bind(ulid())
        .bind(&new_franchise.name)
        .bind(&new_franchise.picture)
        .bind(&new_franchise.bio)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(franchise)
}

pub async fn update_franchise(
    id: &str,
    updated_franchise: &UpdatedFranchise,
) -> ApiResult<Franchise> {
    let query = format!(
        "UPDATE {} SET name = $1, picture = $2, bio = $3, edited = now() WHERE id = $4 RETURNING *",
        String::from(TableNames::Franchise)
    );

    let franchise = sqlx::query_as::<_, DbFranchise>(&query)
        .bind(&updated_franchise.name)
        .bind(&updated_franchise.picture)
        .bind(&updated_franchise.bio)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(franchise)
}
