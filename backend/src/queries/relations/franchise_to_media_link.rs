use super::prelude::*;
use types::{media::*, relations::franchise_to_media_link::*};

pub async fn get_media_from_franchise(franchise_id: &str) -> ApiResult<Vec<Media>> {
    let link_table_name = String::from(TableNames::FranchiseToMediaLink);
    let query = format!(
        r#"
        SELECT media.* FROM {}
        JOIN {} ON media.id = {}.media_id
        WHERE {}.franchise_id = $1
        GROUP BY media.id
    "#,
        String::from(TableNames::Media),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let media = sqlx::query_as::<_, DbMedia>(&query)
        .bind(franchise_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|m| m.into())
        .collect();

    Ok(media)
}

pub async fn create_franchise_to_media_link(
    new_franchise_to_media_link: &NewFranchiseToMediaLink,
) -> ApiResult<()> {
    let query = format!(
        "INSERT INTO {} (id, franchise_id, media_id) VALUES ($1, $2, $3)",
        String::from(TableNames::FranchiseToMediaLink)
    );

    sqlx::query(&query)
        .bind(ulid())
        .bind(&new_franchise_to_media_link.franchise_id)
        .bind(&new_franchise_to_media_link.media_id)
        .execute(pool().await)
        .await?;

    Ok(())
}

pub async fn delete_franchise_to_media_link(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1",
        String::from(TableNames::FranchiseToMediaLink)
    );

    sqlx::query_as::<_, DbFranchiseToMediaLink>(&query)
        .bind(id)
        .fetch_optional(pool().await)
        .await?;

    Ok(())
}
