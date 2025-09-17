use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::LocalBoxFuture;
use sha2::{Sha256, Digest};
use sqlx::{PgPool, Row};
use url::form_urlencoded;

pub struct ApiKey;

impl FromRequest for ApiKey {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = match req.app_data::<actix_web::web::Data<PgPool>>() {
            Some(pool) => pool.clone(),
            None => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorInternalServerError("DB pool not found"))
                });
            }
        };

        let api_key = req
            .headers()
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let business_id = form_urlencoded::parse(req.query_string().as_bytes())
            .find(|(k, _)| k == "business_id")
            .map(|(_, v)| v.to_string());

        Box::pin(async move {
            let api_key = match api_key {
                Some(key) => key,
                None => return Err(actix_web::error::ErrorUnauthorized("Missing API key")),
            };

            let business_id = match business_id {
                Some(id) => id,
                None => return Err(actix_web::error::ErrorUnauthorized("Missing business_id")),
            };

            // Hash the API key
            let hash = Sha256::digest(api_key.as_bytes());
            let hash_hex = format!("{:x}", hash);
            println!("{}", hash_hex);

            // Run DB query
            let row = sqlx::query(
                "SELECT EXISTS(SELECT 1 FROM api_key WHERE key_hash = $1 AND business_id = $2)"
            )
            .bind(&hash_hex)
            .bind(&business_id)
            .fetch_one(pool.get_ref())
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("DB query failed"))?;

            let exists: bool = row.get(0);

            if exists {
                Ok(ApiKey)
            } else {
                Err(actix_web::error::ErrorUnauthorized("Invalid API key for this business"))
            }
        })
    }
}




