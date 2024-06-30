use zero2prod::email_client::subscriptions_confirm_route;

use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let response = reqwest::get(&format!("{}{}", app.address, subscriptions_confirm_route()))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}
