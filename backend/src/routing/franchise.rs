use queries::franchise;
use types::franchise::{Franchise, NewFranchise, UpdatedFranchise};

use super::prelude::*;

pub struct FranchiseRoutes;

#[OpenApi]
impl FranchiseRoutes {
    #[oai(path = "/franchise/:id", method = "get")]
    async fn get_franchise(&self, id: Path<String>) -> ApiResult<Json<Franchise>> {
        let franchise = franchise::get_franchise_by_id(&id.0).await?;

        Ok(Json(franchise))
    }

    #[oai(path = "/franchise", method = "post")]
    async fn create_franchise(&self, data: Json<NewFranchise>) -> ApiResult<Json<Franchise>> {
        let franchise = franchise::create_franchise(&data.0).await?;

        Ok(Json(franchise))
    }

    #[oai(path = "/franchise/:id", method = "patch")]
    async fn update_franchise(
        &self,
        id: Path<String>,
        data: Json<UpdatedFranchise>,
    ) -> ApiResult<Json<Franchise>> {
        let franchise = franchise::update_franchise(&id.0, &data.0).await?;

        Ok(Json(franchise))
    }
}
