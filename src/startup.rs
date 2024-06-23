use crate::{
    configuration::Settings,
    email_client::EmailClient,
    routes::{health_check, health_check_route, subscribe, subscriptions_route},
};

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub async fn build(configuration: Settings) -> Result<Server, std::io::Error> {
    // Application
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address).expect(&format!(
        "Failed to bind to port {}",
        configuration.application.port
    ));

    // Database
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    // Email Client
    let sender_email = configuration
        .email_client
        .sender_email()
        .expect("Invalid sender email address.");

    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    run(listener, connection_pool, email_client)
}

pub fn run(
    listener: TcpListener,
    connection_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    let email_client = web::Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route(&health_check_route(), web::get().to(health_check))
            .route(&subscriptions_route(), web::post().to(subscribe))
            .app_data(connection_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub struct PostRequestHeader {
    pub name: String,
    pub value: String,
}

pub fn header() -> PostRequestHeader {
    PostRequestHeader {
        name: String::from("Content-Type"),
        value: String::from("application/x-www-form-urlencoded"),
    }
}
