use axum::{Router, routing::get};
use thiserror::Error;
use tokio::{io, net::TcpListener};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}

pub fn build_routes() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}

pub async fn get_tcp_listener(addr: &str) -> Result<TcpListener, ServerError> {
    Ok(TcpListener::bind(addr).await?)
}
