// use actix_web::{post, Responder, HttpResponse, web};
// use serde::{Serialize, Deserialize};
// use crate::extractor::ApiKey;
// use crate::generate::gen_uuid;
// use sqlx::Row;
// use sha2::{Sha256, Digest};
// use rand::distr::Alphanumeric;  
// use rand::Rng;

// #[derive(Deserialize)]
// struct CreateWebhookQuery {
//     business_id: String
// }

// #[derive(Deserialize)]
// struct CreateWebhookBody {
//     url: String,
// }

// #[derive(Serialize)]
// pub struct CreateWebhookResponse {
//     id: String,
//     key: String,
//     secret: String,
// }


// #[post("/create_webhook")]
// pub async fn create_webhook(db: web::Data<sqlx::PgPool>, _auth: ApiKey, props: web::Json<CreateWebhookBody>, query: web::Query<CreateWebhookQuery>) -> impl Responder {
//     let webhook_id = gen_uuid().await;

//     let webhook_key: String = rand::rng()
//         .sample_iter(&Alphanumeric)
//         .take(32)
//         .map(char::from)
//         .collect();

//     let mut hasher = Sha256::new();
//     hasher.update(&webhook_key);
//     let webhook_key_secret = format!("{:x}", hasher.finalize());

//         let inserted = sqlx::query(
//         r#"
//         INSERT INTO webhooks (id, business_id, url, secret)
//         VALUES ($1, $2, $3, $4)
//         RETURNING id::text, secret
//         "#,
//     )
//     .persistent(false)
//     .bind(webhook_id)
//     .bind(&query.business_id)
//     .bind(&props.url)
//     .bind(&webhook_key_secret)
//     .fetch_one(db.get_ref())
//     .await;

//      match inserted {
//         Ok(row) => {
//             let resp = CreateWebhookResponse {
//                 id: row.get("id"),
//                 key: webhook_key,
//                 secret: row.get("secret"),
//             };
//             HttpResponse::Ok().json(resp)
//         }
//         Err(e) => HttpResponse::InternalServerError().body(format!("DB insert failed: {}", e)),
//     }
// }

