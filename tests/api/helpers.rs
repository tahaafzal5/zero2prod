use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
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

pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to read configuration");

    // Database
    let host = &configuration.database.host;
    let listener =
        TcpListener::bind(format!("{}:0", host)).expect("Failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://{}:{}", host, port);

    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    // Email Client
    let sender_email = configuration
        .email_client
        .sender_email()
        .expect("Invalid sender email address");

    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let server =
        run(listener, connection_pool.clone(), email_client).expect("Failed to bind address");

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
