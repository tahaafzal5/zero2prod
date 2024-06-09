use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!(
        "{}:{}",
        configuration.database.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect(&format!(
        "Failed to bind to port {}",
        configuration.application.port
    ));

    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect_lazy(connection_string.expose_secret())
        .expect("Failed to connect to Postgres");

    run(listener, connection_pool)?.await
}
