use zero2prod::routes::login_route;
use zero2prod::routes::post::flash_cookie_string;

use crate::helpers::assert_is_redirect_to;
use crate::helpers::spawn_app;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password",
    });

    let response = app.post_login(&login_body).await;

    let flash_cookie = response
        .cookies()
        .find(|c| c.name() == flash_cookie_string())
        .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 303); // 303 is a redirect status code
    assert_is_redirect_to(&response, login_route());
    assert_eq!(flash_cookie.value(), "Authentication failed");
}
