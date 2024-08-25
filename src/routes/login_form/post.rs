use actix_web::{web, HttpResponse};
use reqwest::header::LOCATION;
use secrecy::Secret;

use crate::routes::home_route;

#[derive(serde::Deserialize)]
pub struct FormData {
    _email: String,
    _password: Secret<String>,
}

pub async fn login(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, home_route()))
        .finish()
}
