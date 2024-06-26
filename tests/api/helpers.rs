use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::routes::subscriptions_route;
use zero2prod::startup::{get_connection_pool, header, Application};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
}

impl TestApp {
    pub async fn send_subscription_request(&self, body: String) -> reqwest::Response {
        let post_request_header = header();
        let request = format!("{}{}", &self.address, subscriptions_route());

        reqwest::Client::new()
            .post(request)
            .header(&post_request_header.name, &post_request_header.value)
            .body(body)
            .send()
            .await
            .expect("Failed to execute POST request")
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

pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // Database
    let configuration = {
        let mut config = get_configuration().expect("Failed to read configuration");
        config.database.database_name = Uuid::new_v4().to_string();
        config.application.port = 0;

        config
    };
    configure_database(&configuration.database).await;

    // Application
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    let address = format!(
        "http://{}:{}",
        configuration.database.host,
        application.port()
    );

    // Launch our application in the background
    tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        connection_pool: get_connection_pool(&configuration.database),
    }
}

/*
Create a new database with a new/random name for each test for test isolation.
We then also need to run migrations on this new database.
*/
pub async fn configure_database(config: &DatabaseSettings) {
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
}
