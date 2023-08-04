use reqwest::Url;
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
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();
    // Extract the link from one of the request fields.
    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };
    let raw_confirmation_link = &get_link(&body["HtmlBody"].as_str().unwrap());
    let mut confirmation_link = Url::parse(&raw_confirmation_link).unwrap();
    // Verifying for not checking random API's on the web
    assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
    confirmation_link.set_port(Some(test_app.port)).unwrap();
    dbg!(&confirmation_link.as_str());

    // Act
    let response = reqwest::get(confirmation_link).await.unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}
