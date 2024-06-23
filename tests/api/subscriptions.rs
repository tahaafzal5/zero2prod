use crate::helpers::spawn_app;
use zero2prod::routes::subscriptions_route;
use zero2prod::startup::header;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let post_request_header = header();
    let request = format!("{}{}", app.address, subscriptions_route());
    let body = format!("name=Taha%20Afzal&email=tahaafzal5%40hotmail.com");
    let response = client
        .post(request)
        .header(&post_request_header.name, &post_request_header.value)
        .body(body)
        .send()
        .await
        .expect("Failed to execute POST request");

    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.connection_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "tahaafzal5@hotmail.com");
    assert_eq!(saved.name, "Taha Afzal");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let request = format!("{}{}", app.address, subscriptions_route());

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    let post_request_header = header();
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&request)
            .header(&post_request_header.name, &post_request_header.value)
            .body(invalid_body)
            .send()
            .await
            .expect(&error_message);

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
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    let post_request_header = header();
    for (body, description) in test_cases {
        let response = client
            .post(&format!("{}{}", &app.address, subscriptions_route()))
            .header(&post_request_header.name, &post_request_header.value)
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 Bad Request when the payload was {}",
            description
        );
    }
}
