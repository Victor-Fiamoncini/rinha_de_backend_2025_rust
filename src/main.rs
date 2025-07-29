use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use redis::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CreatePaymentBody {
    #[serde(rename = "correlationId")]
    correlation_id: String,
    amount: f64,
}

#[post("/payments")]
async fn create_payment(
    payload: web::Json<CreatePaymentBody>,
    redis_connection: web::Data<redis::aio::MultiplexedConnection>,
) -> impl Responder {
    let json_string = match serde_json::to_string(&payload.into_inner()) {
        Ok(val) => val,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let mut connection = redis_connection.as_ref().clone();
    let lpush_result: redis::RedisResult<()> = redis::cmd("LPUSH")
        .arg("@payments_queue")
        .arg(json_string)
        .query_async(&mut connection)
        .await;

    match lpush_result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct PaymentsSummaryQuery {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

#[get("/payments-summary")]
async fn get_payments_summary(query: web::Query<PaymentsSummaryQuery>) -> impl Responder {
    let from = query.from;
    let to = query.to;

    HttpResponse::Ok().body(format!("Payments summary from {} to {}", from, to))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let api_port = env::var("API_PORT").expect("API_PORT env must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL env must be set");

    let redis_client = Client::open(redis_url).expect("Failed to create Redis client");

    let redis_connection = redis_client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Failed to connect to Redis");

    let redis_connection = web::Data::new(redis_connection);

    println!(
        "{}",
        format!("🦀 Server started successfully on port {api_port}")
    );

    HttpServer::new(move || {
        App::new()
            .app_data(redis_connection.clone())
            .service(create_payment)
            .service(get_payments_summary)
    })
    .bind(format!("0.0.0.0:{api_port}"))?
    .run()
    .await
}
