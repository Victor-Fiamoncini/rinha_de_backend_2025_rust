use std::env;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreatePaymentBody {
    #[serde(rename = "correlationId")]
    correlation_id: String,
    amount: f64,
}

#[post("/payments")]
async fn create_payment(payload: actix_web::web::Json<CreatePaymentBody>) -> impl Responder {
    let data = payload.into_inner();

    print!(
        "Received payment request: correlation_id={}, amount={}",
        data.correlation_id, data.amount
    );

    HttpResponse::Created()
}

#[derive(Deserialize)]
struct PaymentsSummaryQuery {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}

#[get("/payments-summary")]
async fn get_payments_summary(
    query: actix_web::web::Query<PaymentsSummaryQuery>,
) -> impl Responder {
    let from = query.from;
    let to = query.to;

    HttpResponse::Ok().body(format!("Payments summary from {} to {}", from, to))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api_port = env::var("API_PORT").expect("API_PORT env must be set");

    println!(
        "{}",
        format!("🦀 Server started successfully on port {api_port}")
    );

    HttpServer::new(|| {
        App::new()
            .service(create_payment)
            .service(get_payments_summary)
    })
    .bind(format!("0.0.0.0:{api_port}"))?
    .run()
    .await
}
