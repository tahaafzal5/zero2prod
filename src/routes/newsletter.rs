use actix_web::HttpResponse;

pub async fn publish_newsletter() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn publish_newsletter_route() -> String {
    String::from("/newsletter")
}
