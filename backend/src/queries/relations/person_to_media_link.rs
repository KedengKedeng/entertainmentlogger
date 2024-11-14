use types::{
    media::{DbMedia, Media},
    person::{DbPerson, Person},
    relations::person_to_media_link::{
        DbPersonToMediaLink, NewPersonToMediaLink, PersonToMediaLink,
    },
};

use super::prelude::*;

pub async fn create_person_to_media_link(
    new_link: &NewPersonToMediaLink,
) -> ApiResult<PersonToMediaLink> {
    let query = format!(
        "INSERT INTO {} (id, person_id, media_id, role) VALUES ($1, $2, $3, $4) RETURNING *",
        String::from(TableNames::PersonToMediaLink)
    );

    let person_to_media_link = sqlx::query_as::<_, DbPersonToMediaLink>(&query)
        .bind(ulid())
        .bind(&new_link.person_id)
        .bind(&new_link.media_id)
        .bind(&new_link.role)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(person_to_media_link)
}

pub async fn get_people_from_media(media_id: &str) -> ApiResult<Vec<Person>> {
    let link_table_name = String::from(TableNames::PersonToMediaLink);
    let query = format!(
        r#"
        SELECT people.* FROM {}
        JOIN {} ON people.id = {}.person_id
        WHERE {}.media_id = $1
        GROUP BY people.id
    "#,
        String::from(TableNames::Person),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let people = sqlx::query_as::<_, DbPerson>(&query)
        .bind(media_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect();

    Ok(people)
}

pub async fn get_media_from_person(person_id: &str) -> ApiResult<Vec<Media>> {
    let link_table_name = String::from(TableNames::PersonToMediaLink);
    let query = format!(
        r#"
        SELECT media.* FROM {}
        JOIN {} ON media.id = {}.media_id
        WHERE {}.person_id = $1
        GROUP BY media.id
    "#,
        String::from(TableNames::Media),
        link_table_name,
        link_table_name,
        link_table_name
    );

    let media = sqlx::query_as::<_, DbMedia>(&query)
        .bind(person_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|m| m.into())
        .collect();

    Ok(media)
}

pub async fn delete_person_to_media_link(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1 RETURNING *",
        String::from(TableNames::PersonToMediaLink)
    );

    sqlx::query_as::<_, DbPersonToMediaLink>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?;

    Ok(())
}
