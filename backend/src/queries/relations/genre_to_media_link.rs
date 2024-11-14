use super::prelude::*;
use types::{
    genre::*,
    media::{DbMedia, Media},
    relations::genre_to_media_link::{DbGenreToMediaLink, GenreToMediaLink, NewGenreToMediaLink},
};

pub async fn get_genres_from_media(media_id: &str) -> ApiResult<Vec<Genre>> {
    let link_table_name = String::from(TableNames::GenreToMediaLink);
    let query = format!(
        r#"
        SELECT genres.* FROM {}
        JOIN {} ON genres.name = {}.genre_id
        WHERE {}.media_id = $1
        GROUP BY genres.name
    "#,
        String::from(TableNames::Genre),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let genres = sqlx::query_as::<_, DbGenre>(&query)
        .bind(media_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|g| g.into())
        .collect();

    Ok(genres)
}

pub async fn get_media_from_genre(genre_id: &str) -> ApiResult<Vec<Media>> {
    let link_table_name = String::from(TableNames::GenreToMediaLink);
    let query = format!(
        r#"
        SELECT media.* FROM {}
        JOIN {} ON media.id = {}.media_id
        WHERE {}.genre_id = $1
        GROUP BY media.id
    "#,
        String::from(TableNames::Media),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let media = sqlx::query_as::<_, DbMedia>(&query)
        .bind(genre_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|m| m.into())
        .collect();

    Ok(media)
}

pub async fn create_genre_to_media_link(link: &NewGenreToMediaLink) -> ApiResult<GenreToMediaLink> {
    let query = format!(
        "INSERT INTO {} (id, media_id, genre_id) VALUES ($1, $2, $3) RETURNING *",
        String::from(TableNames::GenreToMediaLink)
    );

    let genre_to_media_link = sqlx::query_as::<_, DbGenreToMediaLink>(&query)
        .bind(ulid())
        .bind(&link.media_id)
        .bind(&link.genre_id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(genre_to_media_link)
}

pub async fn delete_genre_to_media_link(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1 RETURNING *",
        String::from(TableNames::GenreToMediaLink)
    );

    sqlx::query_as::<_, DbGenreToMediaLink>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?;

    Ok(())
}
