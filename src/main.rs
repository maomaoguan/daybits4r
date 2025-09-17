use std::net::SocketAddr;

use axum::{
	routing::{get, post},
	Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::{
	services::ServeDir,
	set_header::SetResponseHeaderLayer,
	cors::{Any, CorsLayer},
};

use daybits4r::*;

#[derive(Serialize, Deserialize)]
struct AndOrRequest {
    left: String,
    right: String,
}

#[derive(Serialize, Deserialize)]
struct DaybitsTextRequest {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct DaybitsResult<T> {
    ok: bool,
    data: T,
}

#[tokio::main]
async fn main() {
    // Allow embedding via iframe and cross-origin host demo.
    let frame_headers = SetResponseHeaderLayer::if_not_present(
        axum::http::header::HeaderName::from_static("x-frame-options"),
        axum::http::HeaderValue::from_static("ALLOWALL"),
    );

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .route("/api/and", post(api_and))
        .route("/api/or", post(api_or))
        .route("/api/first", post(api_first))
        .route("/api/last", post(api_last))
        .route("/api/count", post(api_count));

    let app = Router::new()
        // Static assets: /static/host.html, /static/console1.html, /static/console2.html
        .nest_service("/static", ServeDir::new("static"))
        .merge(api)
        .layer(frame_headers)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn api_and(Json(req): Json<AndOrRequest>) -> Json<DaybitsResult<String>> {
    let result = daybits_and(&req.left, &req.right);
    Json(DaybitsResult { ok: true, data: result })
}

async fn api_or(Json(req): Json<AndOrRequest>) -> Json<DaybitsResult<String>> {
    let result = daybits_or(&req.left, &req.right);
    Json(DaybitsResult { ok: true, data: result })
}

async fn api_first(Json(req): Json<DaybitsTextRequest>) -> Json<DaybitsResult<i64>> {
    let result = daybits_first(&req.text);
    Json(DaybitsResult { ok: true, data: result })
}

async fn api_last(Json(req): Json<DaybitsTextRequest>) -> Json<DaybitsResult<i64>> {
    let result = daybits_last(&req.text);
    Json(DaybitsResult { ok: true, data: result })
}

async fn api_count(Json(req): Json<DaybitsTextRequest>) -> Json<DaybitsResult<i64>> {
    let result = daybits_count(&req.text);
    Json(DaybitsResult { ok: true, data: result })
}

