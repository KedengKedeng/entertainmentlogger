use types::user::*;

use super::prelude::*;

pub async fn get_user_by_id(id: &str) -> ApiResult<User> {
    let query = format!(
        "SELECT * FROM {} WHERE id = $1",
        String::from(TableNames::User)
    );

    let user = sqlx::query_as::<_, DbUser>(&query)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(user)
}

pub async fn get_user_by_email(email: &str) -> ApiResult<User> {
    let query = format!(
        "SELECT * FROM {} WHERE email = $1",
        String::from(TableNames::User)
    );

    let user = sqlx::query_as::<_, DbUser>(&query)
        .bind(email)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(user)
}

pub async fn create_user(new_user: &NewUser) -> ApiResult<User> {
    let query = format!("INSERT INTO {} (id, username, email, password, last_online) VALUES ($1, $2, $3, $4, now()) RETURNING *", String::from(TableNames::User));

    let user = sqlx::query_as::<_, DbUser>(&query)
        .bind(ulid())
        .bind(&new_user.username)
        .bind(&new_user.email)
        .bind(&new_user.password)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(user)
}

pub async fn update_user(id: &str, updated_user: &UpdatedUser) -> ApiResult<User> {
    let query = format!(
        "UPDATE {} SET username = $1, profile_picture = $2, bio = $3 WHERE id = $4 RETURNING *",
        String::from(TableNames::User)
    );

    let user = sqlx::query_as::<_, DbUser>(&query)
        .bind(&updated_user.username)
        .bind(&updated_user.profile_picture)
        .bind(&updated_user.bio)
        .bind(id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(user)
}

pub async fn delete_user(id: &str) -> ApiResult<()> {
    let query = format!(
        "DELETE FROM {} WHERE id = $1",
        String::from(TableNames::User)
    );

    sqlx::query_as::<_, DbUser>(&query)
        .bind(id)
        .fetch_optional(pool().await)
        .await?;

    Ok(())
}
