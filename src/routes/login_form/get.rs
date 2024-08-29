use actix_web::{http::header::ContentType, web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let error_html = match query.0.error {
        None => "".into(),
        // Encode the error message to prevent XSS attacks
        // since the error message is being injected into the HTML
        Some(error_message) => format!(
            "<p><i>{}</i></p>",
            htmlescape::encode_minimal(&error_message)
        ),
    };
    
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">

<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>

<body>
    {error_html}
    <form action="/login" method="post">
        <label>Email Address
            <input type="text" placeholder="Enter your email address" name="email">
        </label>

        <label>Password
            <input type="text" placeholder="Enter password" name="password">
        </label>

        <button type="submit">Login</button>
    </form>
</body>

</html>"#
        ))
}

pub fn login_route() -> String {
    String::from("/login")
}
