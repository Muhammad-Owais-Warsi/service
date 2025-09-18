use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::generate::gen_uuid;
use uuid::Uuid;
use sqlx::Row;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use rand::distr::Alphanumeric;  
use rand::Rng;


#[derive(Deserialize)]
struct CreateBusiness {
    name: String,
}

#[derive(Serialize)]
pub struct CreateBusinessResponse {
    id: String,
    name: String,
}

#[post("/create_business")]
pub async fn create_business(db: web::Data<sqlx::PgPool>, props: web::Json<CreateBusiness>) -> impl Responder {
    let id = gen_uuid().await;
    let name = &props.name;

    let result = sqlx::query(
        r#"
        INSERT INTO business (id, name)
        VALUES ($1, $2)
        RETURNING id, name
        "#
    )
    .persistent(false)
    .bind(id)
    .bind(name)
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(record) => {
            let response = CreateBusinessResponse {
                id: record.get::<String, _>("id"),
                name: record.get::<String, _>("name"),
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            eprintln!("DB insert error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to create business")
        }
    }
    
}


#[derive(Deserialize)]
struct CreateBusinessApiKey {
    id: String,
}

#[derive(Serialize)]
pub struct CreateBusinessApiKeyResponse {
    id: String,
    api_key: String,
    api_key_hash: String,
    created_at: DateTime<Utc>,
}

#[post("/create_api_key")]
pub async fn create_api_keys(db: web::Data<sqlx::PgPool>,props: web::Json<CreateBusinessApiKey>) -> impl Responder {
    let business_id = &props.id;

    // 1. Check business exists
    let business = sqlx::query("SELECT id FROM business WHERE id = $1")
        .persistent(false)
        .bind(business_id)
        .fetch_optional(db.get_ref())
        .await;

    match business {
        Ok(Some(_)) => {
            // 2. Generate key + hash
            let api_key: String = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();

            let mut hasher = Sha256::new();
            hasher.update(&api_key);
            let api_key_hash = format!("{:x}", hasher.finalize());

            let id = Uuid::new_v4().to_string();

            // 3. Insert into api_key table
            let inserted = sqlx::query(
                r#"
                INSERT INTO api_key (id, business_id, key_hash, created_at)
                VALUES ($1, $2, $3, $4)
                RETURNING id, key_hash, created_at
                "#
            )
            .persistent(false)
            .bind(&id)               
            .bind(&business_id)     
            .bind(&api_key_hash)   
            .bind(chrono::Utc::now())
            .fetch_one(db.get_ref())
            .await;

            match inserted {
                Ok(record) => {
                    let response = CreateBusinessApiKeyResponse {
                        id: record.get::<String, _>("id"),
                        api_key, // return the plain key once
                        api_key_hash: record.get::<String, _>("key_hash"),
                        created_at: record.get::<DateTime<Utc>, _>("created_at"),
                    };
                    HttpResponse::Ok().json(response)
                }
                Err(err) => {
                    eprintln!("DB insert error: {:?}", err);
                    HttpResponse::InternalServerError().body("Failed to create API key")
                }
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().body("Business not found")
        }
        Err(err) => {
            eprintln!("DB query error: {:?}", err);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}
