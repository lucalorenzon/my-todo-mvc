use todo_api::{ServerError, build_routes, get_tcp_listener};
use tracing::info;

#[tokio::main]
pub async fn main() -> Result<(), ServerError> {
    tracing_subscriber::fmt::init();
    let tcp_listener = get_tcp_listener("0.0.0.0:3000").await?;
    let app = build_routes();
    info!(
        "Running server on port, {}",
        &tcp_listener.local_addr()?.port()
    );
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
