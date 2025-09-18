// use actix_web::web;
// use hmac::{Hmac, Mac};
// use reqwest::Client;
// use sha2::Sha256;
// use sqlx::{PgPool, Row};
// use tokio::time::{sleep, Duration};
// use hex;

// type HmacSha256 = Hmac<Sha256>;

// pub async fn dispatch_webhooks(pool: web::Data<PgPool>, secret: &str) {
//     let client = Client::new();

//     loop {
//         let mut conn = match pool.acquire().await {
//             Ok(c) => c,
//             Err(e) => {
//                 eprintln!("Failed to acquire connection: {}", e);
//                 sleep(Duration::from_secs(5)).await;
//                 continue;
//             }
//         };

     
//         let query_result = sqlx::query(
//             "SELECT id::text, event, payload::text FROM webhook_logs WHERE status = 'pending' ORDER BY created_at LIMIT 10"
//         )
//         .fetch_all(&mut *conn)
//         .await;

//         let events = match query_result {
//             Ok(rows) => rows,
//             Err(e) => {
//                 eprintln!("Failed to fetch events: {}", e);
//                 sleep(Duration::from_secs(5)).await;
//                 continue;
//             }
//         };

//         for event in events {
//             let id_str: String = event.get("id");
//             let event_type: String = event.get("event");
//             let payload: Option<String> = event.get("payload");
//             let payload = payload.unwrap_or_default();

       
//             let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
//             mac.update(payload.as_bytes());
//             let signature = hex::encode(mac.finalize().into_bytes());

//             let webhook_url = "https://client-server.com/webhooks";

//             // 4. Send webhook
//             let res = client
//                 .post(webhook_url)
//                 .header("X-Signature", &signature)
//                 .header("X-Event-Type", &event_type)
//                 .body(payload.clone())
//                 .send()
//                 .await;

//             // 5. Update status with direct SQL execution
//             let update_sql = match res {
//                 Ok(resp) if resp.status().is_success() => {
//                     format!("UPDATE webhook_logs SET status = 'delivered', delivered_at = now() WHERE id = '{}'", id_str)
//                 }
//                 Ok(resp) => {
//                     format!("UPDATE webhook_logs SET status = 'failed', error = 'HTTP {}', delivered_at = now() WHERE id = '{}'", resp.status(), id_str)
//                 }
//                 Err(e) => {
//                     let safe_error = e.to_string().replace("'", "''");
//                     format!("UPDATE webhook_logs SET status = 'failed', error = '{}', delivered_at = now() WHERE id = '{}'", safe_error, id_str)
//                 }
//             };

//             // Execute update with a fresh connection if needed
//             let _ = sqlx::query(&update_sql)
//                 .execute(&mut *conn)
//                 .await;
//         }

        
//         drop(conn);

//         sleep(Duration::from_secs(5)).await;
//     }
// }