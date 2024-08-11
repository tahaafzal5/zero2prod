use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;
use chrono::Utc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use std::fmt::{Debug, Display};
use uuid::Uuid;

use crate::{
    domain::{NewSubscriber, SubscriberEmail},
    email_client::EmailClient,
};

#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl Debug for SubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeError {
    fn status_code(&self) -> StatusCode {
        match self {
            SubscribeError::ValidationError(_) => StatusCode::BAD_REQUEST,
            SubscribeError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub struct StoreTokenError(sqlx::Error);

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // The compiler transparently casts `&sqlx::Error` into a `&dyn Error`
        Some(&self.0)
    }
}

impl Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while \
            trying to store a subscription token."
        )
    }
}

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;

    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, connection_pool, email_client, base_url),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    connection_pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<String>,
) -> Result<HttpResponse, SubscribeError> {
    let new_subscriber: NewSubscriber =
        form.0.try_into().map_err(SubscribeError::ValidationError)?;

    let subscription_token =
        match get_subscription_token_if_subscriber_exists(&connection_pool, &new_subscriber.email)
            .await
        {
            Ok(Some(token)) => Some(token),
            Ok(None) => None,
            Err(_) => None,
        };

    if subscription_token.is_none() {
        let mut transaction = connection_pool
            .begin()
            .await
            .context("Failed to acquire a Postgres connection from the pool")?;

        let subsriber_id = insert_subscriber(&new_subscriber, &mut transaction)
            .await
            .context("Failed to insert a new subscriber in the database")?;

        let subscription_token = generate_subscription_token();
        store_subscription_token(&mut transaction, &subsriber_id, &subscription_token)
            .await
            .context("Failed to store the confirmation token for a new subscriber")?;

        transaction
            .commit()
            .await
            .context("Failed to commit SQL transaction to store a new subscriber")?;

        send_confirmation_email(
            &email_client,
            &new_subscriber,
            &base_url,
            &subscription_token,
        )
        .await
        .context("Failed to send a confirmation email")?;
    } else {
        send_confirmation_email(
            &email_client,
            &new_subscriber,
            &base_url,
            &subscription_token.unwrap(),
        )
        .await
        .context("Failed to send a confirmation email")?;
    }

    return Ok(HttpResponse::Ok().finish());
}

#[tracing::instrument(
    name = "Getting subscription token if subscriber exists",
    skip(connection_pool, email)
)]
async fn get_subscription_token_if_subscriber_exists(
    connection_pool: &web::Data<PgPool>,
    email: &SubscriberEmail,
) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscription_token \
        FROM subscription_tokens \
        INNER JOIN subscriptions \
        ON subscription_tokens.subscriber_id = subscriptions.id \
        WHERE email=$1 \
        AND status='pending_confirmation'",
        email.as_ref()
    )
    .fetch_optional(connection_pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(result.map(|r| r.subscription_token))
}

#[tracing::instrument(
    name = "Sending a confirmation email",
    skip(email_client, new_subscriber, base_url, subscription_token)
)]
async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: &NewSubscriber,
    base_url: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token={}",
        base_url, subscription_token
    );
    let html_body = format!(
        "Welcome to my newsletter<br />\
                Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );
    let text_body = format!(
        "Welcome to my newsletter!\nClick {} to confirm your subcription.",
        confirmation_link
    );

    email_client
        .send_email(&new_subscriber.email, "Welcome!", &html_body, &text_body)
        .await
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, transaction)
)]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();

    let query = sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at, status) VALUES ($1, $2, $3, $4, 'pending_confirmation')",
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now(),
    );

    transaction.execute(query).await.map_err(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        err
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;

    Ok(subscriber_id)
}

fn generate_subscription_token() -> String {
    let mut rng = thread_rng();

    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(transaction, subscription_token)
)]
async fn store_subscription_token(
    transaction: &mut Transaction<'_, Postgres>,
    subscriber_id: &Uuid,
    subscription_token: &str,
) -> Result<(), StoreTokenError> {
    let query = sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscriber_id, subscription_token) VALUES ($1, $2)"#,
        subscriber_id,
        subscription_token
    );

    transaction.execute(query).await.map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        StoreTokenError(e)
    })?;

    Ok(())
}

pub fn subscriptions_route() -> String {
    String::from("/subscriptions")
}
