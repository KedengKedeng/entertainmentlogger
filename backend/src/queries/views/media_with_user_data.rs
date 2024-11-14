use types::views::media_with_user_data::{DbMediaWithUserData, MediaWithUserData};

use super::prelude::*;

pub async fn get_media_with_user_data(
    media_id: &str,
    user_id: &str,
) -> ApiResult<MediaWithUserData> {
    let query = format!(
        r#"
            SELECT
                {0}.id as media_id,
                {0}.release_date,
                {0}.end_date,
                {0}.media_type,
                {0}.picture,
                {0}.name,
                {0}.bio,
                {0}.scene_count,
                {0}.act_count,
                {1}.id as link_id,
                {1}.date_added,
                {1}.date_started,
                {1}.date_ended,
                {1}.status_type,
                {1}.rating,
                {1}.scenes_seen,
                {1}.acts_seen
            FROM {0}
            LEFT JOIN 
                {1}
            ON {0}.id = {1}.media_id
            WHERE {0}.id = $1 AND {1}.user_id = $2
        "#,
        String::from(TableNames::Media),
        String::from(TableNames::MediaToUserLink)
    );

    let result = sqlx::query_as::<_, DbMediaWithUserData>(&query)
        .bind(media_id)
        .bind(user_id)
        .fetch_one(pool().await)
        .await?
        .into();

    Ok(result)
}

pub async fn get_all_media_with_user_data(user_id: &str) -> ApiResult<Vec<MediaWithUserData>> {
    let query = format!(
        r#"
            SELECT
                {0}.id as media_id,
                {0}.release_date,
                {0}.end_date,
                {0}.media_type,
                {0}.picture,
                {0}.name,
                {0}.bio,
                {0}.scene_count,
                {0}.act_count,
                {1}.id as link_id,
                {1}.date_added,
                {1}.date_started,
                {1}.date_ended,
                {1}.status_type,
                {1}.rating,
                {1}.scenes_seen,
                {1}.acts_seen
            FROM {0}
            LEFT JOIN 
                {1}
            ON {0}.id = {1}.media_id
            WHERE {1}.user_id = $1
        "#,
        String::from(TableNames::Media),
        String::from(TableNames::MediaToUserLink)
    );

    let result = sqlx::query_as::<_, DbMediaWithUserData>(&query)
        .bind(user_id)
        .fetch_all(pool().await)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(result)
}
