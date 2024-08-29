use actix_web::{http::header::ContentType, web, HttpResponse};
use hmac::{Hmac, Mac};
use secrecy::ExposeSecret;

use super::post::HmacSecret;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let tag = hex::decode(self.tag)?;

        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));
        let mut mac =
            Hmac::<sha2::Sha256>::new_from_slice(secret.0.expose_secret().as_bytes()).unwrap();
        mac.update(query_string.as_bytes());
        mac.verify_slice(&tag)?;

        Ok(self.error)
    }
}

pub async fn login_form(
    query: Option<web::Query<QueryParams>>,
    hmac_secret: web::Data<HmacSecret>,
) -> HttpResponse {
    let error_html = match query {
        None => "".into(),
        // Encode the error message to prevent XSS attacks
        // since the error message is being injected into the HTML
        Some(query) => match query.0.verify(&hmac_secret) {
            Ok(error) => {
                format!("<p><i>{}</i></p>", htmlescape::encode_minimal(&error))
            }
            Err(error) => {
                tracing::warn!(
                    error.message = %error,
                    error.cause_chain = ?error,
                    "Failed to verify query parameters using the HMAC tag"
                );
                "".into()
            }
        },
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
