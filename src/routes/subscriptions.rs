use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, connection_pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&form, &connection_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, connection_pool)
)]
pub async fn insert_subscriber(
    form: &FormData,
    connection_pool: &web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection_pool.get_ref())
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {:?}", err);
        err
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(())
}

pub fn subscriptions_route() -> String {
    String::from("/subscriptions")
}
