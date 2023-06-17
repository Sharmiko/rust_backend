use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use bigdecimal::BigDecimal;

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


    let result = sqlx::query!("SELECT * FROM books",)
        .fetch_all(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(result.len(), 1);

    let record = &result[0];
    assert_eq!(record.author, payload.get("author").unwrap().as_str().unwrap());
    assert_eq!(record.title, payload.get("title").unwrap().as_str().unwrap());
    assert_eq!(record.pages, payload.get("pages").unwrap().as_i64().unwrap() as i32);
    assert_eq!(record.price, BigDecimal::from_str(&payload.get("price").unwrap().to_string()).unwrap());
    let naive = NaiveDateTime::from_timestamp_opt(payload.get("published_at").unwrap().as_i64().unwrap(), 0).unwrap();
    assert_eq!(record.published_at, DateTime::<Utc>::from_utc(naive, Utc));
}
