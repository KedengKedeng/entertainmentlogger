use queries::{user, views::media_with_user_data};
use types::{
    user::{NewUser, UpdatedUser, User},
    views::media_with_user_data::MediaWithUserData,
};

use crate::util::hash;

use super::prelude::*;

pub struct UserRoutes;

#[OpenApi]
impl UserRoutes {
    #[oai(path = "/user/:id", method = "get")]
    async fn get_user_by_id(&self, id: Path<String>) -> ApiResult<Json<User>> {
        let user = user::get_user_by_id(&id.0).await?;

        Ok(Json(user))
    }

    #[oai(path = "/user/:email/email", method = "get")]
    async fn get_user_by_email(&self, email: Path<String>) -> ApiResult<Json<User>> {
        let user = user::get_user_by_email(&email.0).await?;

        Ok(Json(user))
    }

    #[oai(path = "/user", method = "post")]
    async fn create_user(&self, mut data: Json<NewUser>) -> ApiResult<Json<User>> {
        data.password = hash(&data.password);
        let user = user::create_user(&data.0).await?;

        Ok(Json(user))
    }

    #[oai(path = "/user/:id", method = "put")]
    async fn update_user(
        &self,
        id: Path<String>,
        data: Json<UpdatedUser>,
    ) -> ApiResult<Json<User>> {
        let user = user::update_user(&id.0, &data.0).await?;

        Ok(Json(user))
    }

    #[oai(path = "/user/:id", method = "delete")]
    async fn delete_user(&self, id: Path<String>) -> ApiResult<()> {
        user::delete_user(&id.0).await?;

        Ok(())
    }

    #[oai(path = "/user/:id/media", method = "get")]
    async fn get_user_media(&self, id: Path<String>) -> ApiResult<Json<Vec<MediaWithUserData>>> {
        let media = media_with_user_data::get_all_media_with_user_data(&id.0).await?;

        Ok(Json(media))
    }
}
