use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[tokio::test]
async fn subscriptions_without_tokens_are_rejected_with_a_400() {
    // Arrange
    let test_app = spawn_app().await;

    // Act
    let response = reqwest::get(&format!("{}/subscriptions/confirm", test_app.address))
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    // Arrange
    let test_app = spawn_app().await;
    let body = "name=sara%20kuzoi&email=sara_kuzoi%40tuta.io";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;

    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(&email_request);

    // Act
    let response = reqwest::get(confirmation_links.html).await.unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // Arrange
    let test_app = spawn_app().await;
    let body = "name=sara%20kuzoi&email=sara_kuzoi%40tuta.io";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(&email_request);

    // Act
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "sara_kuzoi@tuta.io");
    assert_eq!(saved.name, "sara kuzoi");
    assert_eq!(saved.status, "confirmed");
}

#[tokio::test]
async fn clicking_twice_on_the_confirmation_link_confirms_a_subscriber() {
    // Arrange
    let test_app = spawn_app().await;
    let body = "name=sara%20kuzoi&email=sara_kuzoi%40tuta.io";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&test_app.email_server)
        .await;

    test_app.post_subscriptions(body.into()).await;
    let email_request = &test_app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = test_app.get_confirmation_links(&email_request);

    // Act
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    reqwest::get(confirmation_links.plain_text)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    // Assert
    let query = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscriber");

    assert_eq!(query.email, "sara_kuzoi@tuta.io");
    assert_eq!(query.name, "sara kuzoi");
    assert_eq!(query.status, "confirmed");
}
