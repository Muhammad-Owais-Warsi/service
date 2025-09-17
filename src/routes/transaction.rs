use actix_web::{post, web, HttpResponse, Responder};
use crate::extractor::ApiKey;
use serde::{Deserialize, Serialize};
use sqlx::{Row, types::Uuid};
use bigdecimal::BigDecimal;

#[derive(Deserialize)]
struct TransferAccountQuery {
    business_id: String,
}

#[derive(Deserialize)]
struct TransferAccountBody {
    from_account_id: String,
    to_account_id: String,
    amount: BigDecimal,
}

#[derive(Serialize)]
pub struct TransferAccountResponse {
    id: String,
    from_account_id: String,
    to_account_id: String,
    amount: BigDecimal,
    status: String,
}

#[post("/transfer")]
async fn transfer(db: web::Data<sqlx::PgPool>,_auth: ApiKey,props: web::Json<TransferAccountBody>,query: web::Query<TransferAccountQuery>) -> impl Responder {
    
    let mut tx = match db.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to start transaction"),
    };

    
    let updated_rows = sqlx::query(
        r#"
        UPDATE account
        SET balance = balance - $1
        WHERE id = $2 AND balance >= $1
        "#,
    )
    .persistent(false)
    .bind(&props.amount)
    .bind(&props.from_account_id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = updated_rows {
        tx.rollback().await.ok();
        return HttpResponse::InternalServerError().body(format!("DB error: {}", e));
    }

    if updated_rows.unwrap().rows_affected() == 0 {
        tx.rollback().await.ok();
        return HttpResponse::BadRequest().body("Insufficient funds or account not found");
    }

   
    if let Err(e) = sqlx::query(
        r#"
        UPDATE account
        SET balance = balance + $1
        WHERE id = $2
        "#,
    )
        .persistent(false)
    .bind(&props.amount)
    .bind(&props.to_account_id)
    .execute(&mut *tx)
    .await
    {
        tx.rollback().await.ok();
        return HttpResponse::InternalServerError().body(format!("DB error: {}", e));
    }

  
    let txn_id = Uuid::new_v4();
    let record = sqlx::query(
        r#"
        INSERT INTO transaction (id, from_account_id, to_account_id, amount, type, status)
        VALUES ($1, $2, $3, $4, 'transfer', 'success')
        RETURNING id::text, from_account_id::text, to_account_id::text, amount, status
        "#,
    )
        .persistent(false)
    .bind(txn_id)
    .bind(&props.from_account_id)
    .bind(&props.to_account_id)
    .bind(&props.amount)
    .fetch_one(&mut *tx)
    .await;

    match record {
        Ok(row) => {
            let resp = TransferAccountResponse {
                id: row.get("id"),
                from_account_id: row.get("from_account_id"),
                to_account_id: row.get("to_account_id"),
                amount: row.get("amount"),
                status: row.get("status"),
            };

            if let Err(_) = tx.commit().await {
                return HttpResponse::InternalServerError().body("Failed to commit transaction");
            }

            HttpResponse::Ok().json(resp)
        }
        Err(e) => {
            tx.rollback().await.ok();
            HttpResponse::InternalServerError().body(format!("Insert failed: {}", e))
        }
    }
}


#[derive(Deserialize)]
struct CreditAccountQuery {
    business_id: String,
}

#[derive(Deserialize)]
struct CreditAccountBody {
    to_account_id: String,
    amount: BigDecimal,
}

#[derive(Serialize)]
pub struct CreditAccountResponse {
    id: String,
    to_account_id: String,
    amount: BigDecimal,
    status: String,
}

#[post("/credit")]
async fn credit(db: web::Data<sqlx::PgPool>,_auth: ApiKey, props: web::Json<CreditAccountBody>,query: web::Query<CreditAccountQuery>) -> impl Responder {
    let mut tx = match db.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to start transaction"),
    };

    let tx_id = Uuid::new_v4().to_string();


    let update_result = sqlx::query(
        r#"
        UPDATE account
        SET balance = balance + $1
        WHERE id = $2 AND business_id = $3
        "#,
    )
    .persistent(false)
    .bind(&props.amount)
    .bind(&props.to_account_id)
    .bind(&query.business_id)
    .execute(&mut *tx)
    .await;

    match update_result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                let _ = tx.rollback().await;
                return HttpResponse::BadRequest().body("Account not found for this business");
            }

          
            let insert_res = sqlx::query(
                r#"
                INSERT INTO transaction (id, to_account_id, amount, type, status)
                VALUES ($1, $2, $3, 'credit', 'success')
                "#,
            )
            .persistent(false)
            .bind(&tx_id)
            .bind(&props.to_account_id)
            .bind(&props.amount)
            .execute(&mut *tx)
            .await;

            if let Err(_) = insert_res {
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().body("Failed to insert transaction");
            }

            if let Err(_) = tx.commit().await {
                return HttpResponse::InternalServerError().body("Failed to commit transaction");
            }

            HttpResponse::Ok().json(CreditAccountResponse {
                id: tx_id,
                to_account_id: props.to_account_id.clone(),
                amount: props.amount.clone(),
                status: "success".to_string(),
            })
        }
        Err(_) => {
            let _ = tx.rollback().await;
            HttpResponse::InternalServerError().body("Database error while crediting account")
        }
    }
}




#[derive(Deserialize)]
struct DebitAccountQuery {
    business_id: String,
}

#[derive(Deserialize)]
struct DebitAccountBody {
    from_account_id: String,
    amount: BigDecimal,
}

#[derive(Serialize)]
pub struct DebitAccountResponse {
    id: String,
    from_account_id: String,
    amount: BigDecimal,
    status: String,
}

#[post("/debit")]
async fn debit(db: web::Data<sqlx::PgPool>,_auth: ApiKey, props: web::Json<DebitAccountBody>,query: web::Query<DebitAccountQuery>) -> impl Responder {
    let mut tx = match db.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to start transaction"),
    };

    let tx_id = Uuid::new_v4().to_string();


    let update_result = sqlx::query(
        r#"
        UPDATE account
        SET balance = balance - $1
        WHERE id = $2 AND business_id = $3
        "#,
    )
    .persistent(false)
    .bind(&props.amount)
    .bind(&props.from_account_id)
    .bind(&query.business_id)
    .execute(&mut *tx)
    .await;

    match update_result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                let _ = tx.rollback().await;
                return HttpResponse::BadRequest().body("Account not found for this business");
            }

          
            let insert_res = sqlx::query(
                r#"
                INSERT INTO transaction (id, from_account_id, amount, type, status)
                VALUES ($1, $2, $3, 'debit', 'success')
                "#,
            )
            .persistent(false)
            .bind(&tx_id)
            .bind(&props.from_account_id)
            .bind(&props.amount)
            .execute(&mut *tx)
            .await;

            if let Err(_) = insert_res {
                let _ = tx.rollback().await;
                return HttpResponse::InternalServerError().body("Failed to insert transaction");
            }

            if let Err(_) = tx.commit().await {
                return HttpResponse::InternalServerError().body("Failed to commit transaction");
            }

            HttpResponse::Ok().json(DebitAccountResponse {
                id: tx_id,
                from_account_id: props.from_account_id.clone(),
                amount: props.amount.clone(),
                status: "success".to_string(),
            })
        }
        Err(_) => {
            let _ = tx.rollback().await;
            HttpResponse::InternalServerError().body("Database error while crediting account")
        }
    }
}
