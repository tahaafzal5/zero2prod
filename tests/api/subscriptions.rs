use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};
use zero2prod::email_client::email_route;

use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;

    Mock::given(path(email_route()))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let body = format!("name=Taha%20Afzal&email=tahaafzal5%40hotmail.com");

    // Act
    let response = app.send_subscription_request(body.into()).await;

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_persists_the_new_subscriber() {
    // Arrange
    let app = spawn_app().await;

    Mock::given(path(email_route()))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let body = format!("name=Taha%20Afzal&email=tahaafzal5%40hotmail.com");

    // Act
    app.send_subscription_request(body.into()).await;

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.connection_pool)
        .await
        .expect("Failed to fetch saved subscription");

    // Assert
    assert_eq!(saved.email, "tahaafzal5@hotmail.com");
    assert_eq!(saved.name, "Taha Afzal");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app.send_subscription_request(invalid_body.into()).await;

        // Assert
        assert!(response.status().is_client_error());
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        // Act
        let response = app.send_subscription_request(body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}",
            description
        );
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path(email_route()))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    app.send_subscription_request(body.into()).await;

    // Assert
    // Mock asserts on drop
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path(email_route()))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Act
    app.send_subscription_request(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    // Assert
    // Both links should be identical
    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn subscribe_twice_when_status_is_pending_confirmation_sends_another_email_reusing_the_subscription_token(
) {
    // First Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path(email_route()))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // First Act
    app.send_subscription_request(body.into()).await;

    let first_email_request = &app.email_server.received_requests().await.unwrap()[0];
    let first_confirmation_links = app.get_confirmation_links(&first_email_request);

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&app.connection_pool)
        .await
        .expect("Failed to fetch saved subscription");

    // First Assert
    assert_eq!(saved.status, "pending_confirmation");

    // Second Arrange
    Mock::given(path(email_route()))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // Second Act
    app.send_subscription_request(body.into()).await;
    let second_email_request = &app.email_server.received_requests().await.unwrap()[0];
    let second_confirmation_links = app.get_confirmation_links(&second_email_request);

    // Second Assert
    assert_eq!(
        first_confirmation_links.html,
        second_confirmation_links.html
    );
    assert_eq!(
        first_confirmation_links.plain_text,
        second_confirmation_links.plain_text
    );
}

#[tokio::test]
async fn subscribing_twice_when_status_is_already_confirmed_returns_a_500() {
    // First Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path(email_route()))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    // First Act
    app.send_subscription_request(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];

    let confirmation_links = app.get_confirmation_links(&email_request);

    // "Click" the confirmation link in the email
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("SELECT status FROM subscriptions")
        .fetch_one(&app.connection_pool)
        .await
        .expect("Failed to fetch saved subscription status");

    // First Assert
    assert_eq!(saved.status, "confirmed");

    // Second Arrange
    Mock::given(path(email_route()))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&app.email_server)
        .await;

    // Second Act
    let result = app.send_subscription_request(body.into()).await;

    // Second Assert
    assert_eq!(result.status().as_u16(), 500);
}

#[tokio::test]
async fn subscribe_fails_if_there_is_a_fatal_database_error() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Sabotage the database
    sqlx::query!("ALTER TABLE subscription_tokens DROP COLUMN subscription_token;")
        .execute(&app.connection_pool)
        .await
        .unwrap();

    // Act
    let response = app.send_subscription_request(body.into()).await;

    // Assert
    assert_eq!(response.status().as_u16(), 500);
}
