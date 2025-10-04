use reqwest::Client;
use todo_api::{ServerError, build_routes, get_tcp_listener};

#[tokio::test]
async fn should_return_hello_world_on_root_path() {
    let running_port = spawn_app().await.expect("Cannot run spawn server");
    let http_client = Client::new();
    let response = http_client
        .get(format!("http://localhost:{}", running_port))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(
        "Hello, World!",
        response
            .text()
            .await
            .expect("Impossible to read the body of the response")
    );
}

async fn spawn_app() -> Result<u16, ServerError> {
    let tcp_listener = get_tcp_listener("0.0.0.0:0").await?;
    let running_port = tcp_listener.local_addr()?.port();
    let app = build_routes();
    tokio::spawn(async move { axum::serve(tcp_listener, app).await });

    Ok(running_port)
}
