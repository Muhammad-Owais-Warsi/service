use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::generate::gen_uuid;
use sqlx::Row;
use crate::extractor::ApiKey;
use bigdecimal::BigDecimal;


#[derive(Deserialize)]
struct CreateAccountQuery {
    business_id: String,
}

#[derive(Deserialize)]
struct CreateAccountBody {
    name: String,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    id: String,
    name: String,
}


#[post("/create_account")]
pub async fn create_account(db: web::Data<sqlx::PgPool>, _auth: ApiKey, props: web::Json<CreateAccountBody>, query: web::Query<CreateAccountQuery>) -> impl Responder {
    let id = gen_uuid().await;
    let business_id = &query.business_id;
    let name = &props.name;

    let result = sqlx::query(
        r#"
        INSERT INTO account (id, name, business_id)
        VALUES ($1, $2, $3)
        RETURNING id, name
        "#
    )
    .persistent(false)
    .bind(id)
    .bind(name)
    .bind(business_id)
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(record) => {
            let response = CreateAccountResponse {
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
struct GetAccountBalance {
    id: String,
}

#[derive(Serialize)]
pub struct GetAccountBalanceResponse {
    balance: BigDecimal,
}

#[get("/get_balance")]
pub async fn get_account_balance(db: web::Data<sqlx::PgPool>, _auth: ApiKey, props: web::Json<GetAccountBalance>) -> impl Responder {
    let id = &props.id;

  
    let result = sqlx::query(
        r#"
        SELECT balance
        FROM account
        WHERE id = $1
        "#
    )
    .persistent(false)
    .bind(id)
    .fetch_optional(db.get_ref())
    .await;

    match result {
        Ok(Some(record)) => {
            let balance: BigDecimal = record.get("balance");
            let response = GetAccountBalanceResponse {
                balance: balance,
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => {
            HttpResponse::NotFound().body("Account not found")
        }
        Err(err) => {
            eprintln!("DB query error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to fetch account balance")
        }
    }
}

