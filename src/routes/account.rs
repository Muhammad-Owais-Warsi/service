use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::generate::gen_uuid;
use sqlx::Row;



#[derive(Deserialize)]
struct CreateAccount {
    business_id: String,
    name: String,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    id: String,
    name: String,
}


#[post("/create_account")]
pub async fn create_account(db: web::Data<sqlx::PgPool>, props: web::Json<CreateAccount>) -> impl Responder {
    let id = gen_uuid().await;
    let business_id = &props.business_id;
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
    balance: String
}

#[get("/get_balance")]
pub async fn get_account_balance(db: web::Data<sqlx::PgPool>, props: web::Json<GetAccountBalance>) -> impl Responder {
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
            // Convert numeric balance to string
            let balance_str = record.get::<String, _>("balance");
            let response = GetAccountBalanceResponse {
                balance: balance_str,
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => {
            // Account not found
            HttpResponse::NotFound().body("Account not found")
        }
        Err(err) => {
            eprintln!("DB query error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to fetch account balance")
        }
    }
}
