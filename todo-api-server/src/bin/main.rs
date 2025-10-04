use todo_api::{ServerError, build_routes, get_tcp_listener};

#[tokio::main]
pub async fn main() -> Result<(), ServerError> {
    let tcp_listener = get_tcp_listener("0.0.0.0:3000").await?;
    let app = build_routes();
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
