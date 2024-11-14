use queries::{media, views::media_with_user_data};
use types::{media::{Media, NewMedia, UpdatedMedia}, views::media_with_user_data::MediaWithUserData};

use super::prelude::*;

pub struct MediaRoutes;

#[OpenApi]
impl MediaRoutes {
    #[oai(path = "/media/:id", method = "get")]
    async fn get_media(&self, id: Path<String>) -> ApiResult<Json<Media>> {
        let media = media::get_media_by_id(&id.0).await?;

        Ok(Json(media))
    }

    #[oai(path = "/media", method = "post")]
    async fn create_media(&self, data: Json<NewMedia>) -> ApiResult<Json<Media>> {
        let media = media::create_media(&data.0).await?;

        Ok(Json(media))
    }

    #[oai(path = "/media/:id", method = "patch")]
    async fn update_media(&self, id: Path<String>, data: Json<UpdatedMedia>) -> ApiResult<Json<Media>> {
        let media = media::update_media(&id.0, &data.0).await?;

        Ok(Json(media))
    }

    #[oai(path = "/media/:id", method = "delete")]
    async fn delete_media(&self, id: Path<String>) -> ApiResult<()> {
        media::delete_media(&id.0).await?;

        Ok(())
    }

    #[oai(path = "/media/:media_id/with-user-data/:user_id", method = "get")]
    async fn get_media_with_user_data(&self, media_id: Path<String>, user_id: Path<String>) -> ApiResult<Json<MediaWithUserData>> {
        let media = media_with_user_data::get_media_with_user_data(&media_id.0, &user_id.0).await?;

        Ok(Json(media))
    }
}