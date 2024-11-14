use queries::media;
use types::media::{Media, NewMedia, UpdatedMedia};

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
}