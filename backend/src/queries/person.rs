use types::person::{DbPerson, NewPerson, Person, UpdatedPerson};

use super::prelude::*;

pub async fn create_person(new_person: &NewPerson) -> ApiResult<Person> {
    let query = format!(
        "INSERT INTO {} (id, name, picture, bio) VALUES ($1, $2, $3, $4) RETURNING *",
        String::from(TableNames::Person)
    );

    let person = sqlx::query_as::<_, DbPerson>(&query)
        .bind(ulid())
        .bind(&new_person.name)
        .bind(&new_person.picture)
        .bind(&new_person.bio)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(person)
}

pub async fn get_person_by_id(id: &str) -> ApiResult<Person> {
    let query = format!(
        "SELECT * FROM {} WHERE id = $1",
        String::from(TableNames::Person)
    );

    let person = sqlx::query_as::<_, DbPerson>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(person)
}

pub async fn get_all_people() -> ApiResult<Vec<Person>> {
    let query = format!("SELECT * FROM {}", String::from(TableNames::Person));

    let people = sqlx::query_as::<_, DbPerson>(&query)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|person| person.into())
        .collect();

    Ok(people)
}

pub async fn update_person(id: &str, updated_person: &UpdatedPerson) -> ApiResult<Person> {
    let query = format!(
        "UPDATE {} SET name = $1, picture = $2, bio = $3 WHERE id = $4 RETURNING *",
        String::from(TableNames::Person)
    );

    let person = sqlx::query_as::<_, DbPerson>(&query)
        .bind(&updated_person.name)
        .bind(&updated_person.picture)
        .bind(&updated_person.bio)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(person)
}

pub async fn search_people_by_name(name: &str) -> ApiResult<Vec<Person>> {
    let query = format!(
        "SELECT * FROM {} WHERE name ILIKE $1",
        String::from(TableNames::Person)
    );

    let people = sqlx::query_as::<_, DbPerson>(&query)
        .bind(format!("%{}%", name))
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(|person| person.into())
        .collect();

    Ok(people)
}

pub async fn delete_person(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1 RETURNING *",
        String::from(TableNames::Person)
    );

    sqlx::query_as::<_, DbPerson>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?;

    Ok(())
}
