mod db;
mod routes;
mod generate;
mod extractor;

use db::init_db;
use routes::business::{create_business, create_api_keys};
use routes::account::{create_account, get_account_balance};
use routes::transaction::{transfer, credit, debit};
use actix_web::{web, App, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    "Hey there!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "postgresql://postgres.fvfitzrdtcmcadedwqss:new_password786@@aws-0-ap-southeast-1.pooler.supabase.com:6543/postgres?pgbouncer=true";
    let db = init_db(&db_url).await;

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .service(create_business)
            .service(create_api_keys)
            .service(create_account)
            .service(get_account_balance)
            .service(transfer)
            .service(credit)
            .service(debit)
            .app_data(db_data.clone()) 
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
