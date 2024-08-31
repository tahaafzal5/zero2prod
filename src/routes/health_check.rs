use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn health_check_route() -> &'static str {
    "/health_check"
}
