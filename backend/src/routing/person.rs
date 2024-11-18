use queries::person;
use types::person::{NewPerson, Person};

use super::prelude::*;

pub struct PersonRoutes;

#[OpenApi]
impl PersonRoutes {
    #[oai(path = "/person", method = "get")]
    async fn get_all_people(&self) -> ApiResult<Json<Vec<Person>>> {
        let persons = person::get_all_people().await?;

        Ok(Json(persons))
    }

    #[oai(path = "/person/:id", method = "get")]
    async fn get_person_by_id(&self, id: Path<String>) -> ApiResult<Json<Person>> {
        let person = person::get_person_by_id(&id.0).await?;

        Ok(Json(person))
    }

    #[oai(path = "/person", method = "post")]
    async fn create_person(&self, data: Json<NewPerson>) -> ApiResult<Json<Person>> {
        let person = person::create_person(&data.0).await?;

        Ok(Json(person))
    }

    #[oai(path = "/person/:name/search", method = "get")]
    async fn search_people_by_name(&self, name: Path<String>) -> ApiResult<Json<Vec<Person>>> {
        let people = person::search_people_by_name(&name.0).await?;

        Ok(Json(people))
    }

    #[oai(path = "/person/:id", method = "delete")]
    async fn delete_person(&self, id: Path<String>) -> ApiResult<()> {
        person::delete_person(&id.0).await?;

        Ok(())
    }
}