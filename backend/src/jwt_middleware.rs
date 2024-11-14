use jsonwebtoken::{ Algorithm, DecodingKey, Validation };
use poem::{ FromRequest, Request, RequestBody, Result };
use poem_openapi::payload::PlainText;
use serde::{ Deserialize, Serialize };

use crate::{ error::ApiErrorResp, CONFIG };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub username: String,
    pub email: String,
    pub id: String,
    pub exp: i64,
}

impl<'a> FromRequest<'a> for UserSession{
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let cookies = req.cookie();

        let token = cookies
            .get("token")
            .ok_or_else(|| ApiErrorResp::Unauthorized(PlainText("Unauthorized".to_string())))?;

        let session_result = jsonwebtoken::decode::<Self>(
            token.value_str(),
            &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS512)
        );
        
        let session = session_result.map_err(|e|
            ApiErrorResp::Unauthorized(PlainText(e.to_string()))
        )?;

        tracing::debug!("Successfully parsed user JWT: {:?}", session);

        Ok(session.claims)
    }
}