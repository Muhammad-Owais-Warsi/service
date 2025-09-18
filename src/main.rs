mod db;
mod routes;
mod generate;
mod extractor;
mod dispatcher;


use db::init_db;
use dispatcher::dispatch_webhooks;
use routes::business::{create_business, create_api_keys};
use routes::account::{create_account, get_account_balance};
use routes::transaction::{transfer, credit, debit};
// use routes::webhook::{create_webhook};
use actix_web::{web, App, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    "Hey there!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "postgresql://postgres.fvfitzrdtcmcadedwqss:new_password786@@aws-0-ap-southeast-1.pooler.supabase.com:6543/postgres?pgbouncer=true";
    let db = init_db(&db_url).await;

    let db_data = web::Data::new(db);

    // let dispatcher_db = db_data.clone();
    // tokio::spawn(async move {
    //     dispatch_webhooks(dispatcher_db, "super-secret-key").await;
    // });

    HttpServer::new(move || {
        App::new()
            .service(create_business)
            .service(create_api_keys)
            .service(create_account)
            .service(get_account_balance)
            .service(transfer)
            .service(credit)
            .service(debit)
            // .service(create_webhook)
            .app_data(db_data.clone()) 
            .route("/hey", web::get().to(manual_hello))
    })
 .bind(("0.0.0.0", 8080))?

    .run()
    .await
}
