use axum::{Json, Router, http::StatusCode, routing::get};
use serde_json::json;
use thiserror::Error;
use tokio::{io, net::TcpListener};
use tracing::instrument;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}

pub fn build_routes() -> Router {
    Router::new().route(
        "/health",
        get(|| async { (StatusCode::OK, Json(json!({ "status": "ok" }))) }),
    )
}

#[instrument]
pub async fn get_tcp_listener(addr: &str) -> Result<TcpListener, ServerError> {
    Ok(TcpListener::bind(addr).await?)
}
