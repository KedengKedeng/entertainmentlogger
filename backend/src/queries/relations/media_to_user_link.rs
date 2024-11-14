use super::prelude::*;
use types::{media::*, relations::media_to_user_link::*};

pub async fn get_media_from_user(user_id: &str) -> ApiResult<Vec<Media>> {
    let link_table_name = String::from(TableNames::MediaToUserLink);
    let query = format!(
        r#"
        SELECT media.* FROM {}
        JOIN {} ON media.id = {}.media_id
        WHERE {}.user_id = $1
        GROUP BY media.id
    "#,
        String::from(TableNames::Media),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let media = sqlx::query_as::<_, DbMedia>(&query)
        .bind(user_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|m| m.into())
        .collect();

    Ok(media)
}

pub async fn create_media_to_user_link(link: &NewMediaToUserLink) -> ApiResult<MediaToUserLink> {
    let query = format!(
        "INSERT INTO {} (id, user_id, media_id, date_started, date_ended, status_type, rating, scenes_seen, acts_seen) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
        String::from(TableNames::MediaToUserLink)
    );

    let media_to_user_link = sqlx::query_as::<_, DbMediaToUserLink>(&query)
        .bind(ulid())
        .bind(&link.user_id)
        .bind(&link.media_id)
        .bind(link.date_started)
        .bind(link.date_ended)
        .bind(i8::from(&link.status_type))
        .bind(link.rating)
        .bind(link.scenes_seen)
        .bind(link.acts_seen)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(media_to_user_link)
}

pub async fn update_media_to_user_link(
    id: &str,
    updated_media_to_user_link: &UpdatedMediaToUserLink,
) -> ApiResult<MediaToUserLink> {
    let query = format!(
        "UPDATE {} SET date_started = $1, date_ended = $2, status_type = $3, rating = $4, scenes_seen = $5, acts_seen = $6 WHERE id = $7 RETURNING *",
        String::from(TableNames::MediaToUserLink)
    );

    let media_to_user_link = sqlx::query_as::<_, DbMediaToUserLink>(&query)
        .bind(updated_media_to_user_link.date_started)
        .bind(updated_media_to_user_link.date_ended)
        .bind(i8::from(&updated_media_to_user_link.status_type))
        .bind(updated_media_to_user_link.rating)
        .bind(updated_media_to_user_link.scenes_seen)
        .bind(updated_media_to_user_link.acts_seen)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(media_to_user_link)
}

pub async fn delete_media_to_user_link(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1",
        String::from(TableNames::MediaToUserLink)
    );

    sqlx::query_as::<_, DbMediaToUserLink>(&query)
        .bind(id)
        .fetch_optional(pool().await)
        .await?;

    Ok(())
}
