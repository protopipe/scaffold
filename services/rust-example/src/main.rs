use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    upstream_base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct HelloResponse {
    id: Uuid,
    message: String,
    upstream_message: String,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct UpstreamResponse {
    message: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[get("/ready")]
async fn ready(state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let upstream = fetch_upstream_message(&state).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "upstream_message": upstream.message
    })))
}

#[get("/hello")]
async fn hello(state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let upstream = fetch_upstream_message(&state).await?;

    Ok(HttpResponse::Ok().json(HelloResponse {
        id: Uuid::new_v4(),
        message: "hello world".to_string(),
        upstream_message: upstream.message,
        timestamp: Utc::now(),
    }))
}

async fn fetch_upstream_message(state: &AppState) -> actix_web::Result<UpstreamResponse> {
    let upstream_url = format!("{}/external/message", state.upstream_base_url);

    state
        .client
        .get(upstream_url)
        .send()
        .await
        .map_err(actix_web::error::ErrorBadGateway)?
        .error_for_status()
        .map_err(actix_web::error::ErrorBadGateway)?
        .json::<UpstreamResponse>()
        .await
        .map_err(actix_web::error::ErrorBadGateway)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let upstream_base_url =
        env::var("UPSTREAM_BASE_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());

    let state = AppState {
        upstream_base_url,
        client: reqwest::Client::new(),
    };

    log::info!("starting rust-example on {bind_addr}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(health)
            .service(ready)
            .service(hello)
    })
    .bind(bind_addr)?
    .run()
    .await
}
