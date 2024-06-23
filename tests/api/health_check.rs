use crate::helpers::spawn_app;
use zero2prod::routes::health_check_route;

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares us from having to specify the `#[test]` attribute.
#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let request = format!("{}{}", app.address, health_check_route());
    let response = client
        .get(request)
        .send()
        .await
        .expect("Failed to execute GET request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
