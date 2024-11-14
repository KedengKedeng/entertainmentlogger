use super::prelude::*;
use crate::{
    error::ApiResult,
    types::genre::{DbGenre, Genre, NewGenre},
};

pub async fn create_genre(new_genre: &NewGenre) -> ApiResult<Genre> {
    let query = format!(
        "INSERT INTO {} (name) VALUES ($1) RETURNING *",
        String::from(TableNames::Genre)
    );

    let genre = sqlx::query_as::<_, DbGenre>(&query)
        .bind(new_genre.name.to_lowercase())
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(genre)
}

pub async fn get_all_genres() -> ApiResult<Vec<Genre>> {
    let query = format!("SELECT * FROM {}", String::from(TableNames::Genre));

    let genres = sqlx::query_as::<_, DbGenre>(&query)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|genre| genre.into())
        .collect();

    Ok(genres)
}

pub async fn search_genres_by_name(name: &str) -> ApiResult<Vec<Genre>> {
    let query = format!(
        "SELECT * FROM {} WHERE name ILIKE $1",
        String::from(TableNames::Genre)
    );

    let genres = sqlx::query_as::<_, DbGenre>(&query)
        .bind(format!("%{}%", name.to_lowercase()))
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|genre| genre.into())
        .collect();

    Ok(genres)
}

pub async fn delete_genre(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1 RETURNING *",
        String::from(TableNames::Genre)
    );

    sqlx::query_as::<_, DbGenre>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?;

    Ok(())
}
