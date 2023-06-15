use std::collections::HashMap;

use rust_backend::test_app::TestApp;
use rust_backend::configuration::get_configuration;

#[tokio::test]
async fn test_library_book_insert() {
    let test_app = TestApp::spawn_app().await;

    let configuration = get_configuration().expect("Failed to read configuration");
    let client = reqwest::Client::new();


    let payload = serde_json::json!({
        "author": "Kafka",
        "title": "Metamorphosis",
        "pages": 123,
        "price": 9.99,
        "published_at": 1686777831
    });

    let response = client
        .post(&format!("{}/book", &test_app.address))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload).unwrap())
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);
}
