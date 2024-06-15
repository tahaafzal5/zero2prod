use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::routes::{health_check_route, subscriptions_route};
use zero2prod::startup::{header, run};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
}

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

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to read configuration");

    let host = &configuration.database.host;
    let listener =
        TcpListener::bind(format!("{}:0", host)).expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://{}:{}", host, port);

    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");

    // Launch our application in the background
    tokio::spawn(server);

    TestApp {
        address,
        connection_pool,
    }
}

/*
Create a new database with a new/random name for each test for test isolation.
We then also need to run migrations on this new database.
*/
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    let query = format!(r#"CREATE DATABASE "{}";"#, config.database_name);

    connection
        .execute(query.as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    // Defaults to "./migrations"
    sqlx::migrate!()
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}
