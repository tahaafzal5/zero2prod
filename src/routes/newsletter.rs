use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

pub async fn publish_newsletter(body: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn publish_newsletter_route() -> String {
    String::from("/newsletter")
}
