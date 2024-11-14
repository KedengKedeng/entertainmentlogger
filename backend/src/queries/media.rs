use super::prelude::*;
use types::media::*;

pub async fn get_media_by_id(id: &str) -> ApiResult<Media> {
    let query = format!(
        "SELECT * FROM {} WHERE id = $1",
        String::from(TableNames::Media)
    );

    let media = sqlx::query_as::<_, DbMedia>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(media)
}

pub async fn create_media(new_media: &NewMedia) -> ApiResult<Media> {
    let query = format!(
        "INSERT INTO {} (id, release_date, end_date, media_type, picture, name, bio, scene_count, act_count) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
        String::from(TableNames::Media)
    );

    let media = sqlx::query_as::<_, DbMedia>(&query)
        .bind(ulid())
        .bind(new_media.release_date)
        .bind(new_media.end_date)
        .bind(i8::from(&new_media.media_type))
        .bind(&new_media.picture)
        .bind(&new_media.name)
        .bind(&new_media.bio)
        .bind(new_media.scene_count)
        .bind(new_media.act_count)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(media)
}

pub async fn update_media(id: &str, update_media: &UpdatedMedia) -> ApiResult<Media> {
    let query = format!(
        "UPDATE {} SET release_date = $1, end_date = $2, picture = $3, name = $4, bio = $5, scene_count = $6, act_count = $7 WHERE id = $8 RETURNING *",
        String::from(TableNames::Media)
    );

    let media = sqlx::query_as::<_, DbMedia>(&query)
        .bind(update_media.release_date)
        .bind(update_media.end_date)
        .bind(&update_media.picture)
        .bind(&update_media.name)
        .bind(&update_media.bio)
        .bind(update_media.scene_count)
        .bind(update_media.act_count)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(media)
}

pub async fn delete_media(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1",
        String::from(TableNames::Media)
    );

    sqlx::query(&query)
        .bind(id)
        .execute(pool().await)
        .await?;

    Ok(())
}