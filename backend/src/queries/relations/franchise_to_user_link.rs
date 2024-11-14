use super::prelude::*;
use types::{franchise::*, relations::franchise_to_user_link::*};

pub async fn get_franchises_from_user(user_id: &str) -> ApiResult<Vec<Franchise>> {
    let link_table_name = String::from(TableNames::FranchiseToUserLink);
    let query = format!(
        r#"
        SELECT franchises.* FROM {}
        JOIN {} ON franchises.id = {}.franchise_id
        WHERE {}.user_id = $1
        GROUP BY franchises.id
    "#,
        String::from(TableNames::Franchise),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let franchises = sqlx::query_as::<_, DbFranchise>(&query)
        .bind(user_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|f| f.into())
        .collect();

    Ok(franchises)
}

pub async fn create_franchise_to_user_link(
    link: &NewFranchiseToUserLink,
) -> ApiResult<FranchiseToUserLink> {
    let query = format!(
        "INSERT INTO {} (id, user_id, franchise_id) VALUES ($1, $2, $3) RETURNING *",
        String::from(TableNames::FranchiseToUserLink)
    );

    let franchise_to_user_link = sqlx::query_as::<_, DbFranchiseToUserLink>(&query)
        .bind(ulid())
        .bind(&link.user_id)
        .bind(&link.franchise_id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(franchise_to_user_link)
}

pub async fn delete_franchise_to_user_link(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1",
        String::from(TableNames::FranchiseToUserLink)
    );

    sqlx::query_as::<_, DbFranchiseToUserLink>(&query)
        .bind(id)
        .fetch_optional(pool().await)
        .await?;

    Ok(())
}
