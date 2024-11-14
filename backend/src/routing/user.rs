use queries::user;
use types::user::{NewUser, UpdatedUser, User};

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
    async fn create_user(&self, data: Json<NewUser>) -> ApiResult<Json<User>> {
        let user = user::create_user(&data.0).await?;

        Ok(Json(user))
    }

    #[oai(path = "/user/:id", method = "put")]
    async fn update_user(&self, id: Path<String>, data: Json<UpdatedUser>) -> ApiResult<Json<User>> {
        let user = user::update_user(&id.0, &data.0).await?;

        Ok(Json(user))
    }

    #[oai(path = "/user/:id", method = "delete")]
    async fn delete_user(&self, id: Path<String>) -> ApiResult<()> {
        user::delete_user(&id.0).await?;

        Ok(())
    }
}
