use chrono::{DateTime, Utc,};
use chrono::serde::ts_seconds;
use bigdecimal::BigDecimal;


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Book {
    pub author: String,
    pub title: String,
    pub pages: i32,
    pub price: BigDecimal,
    #[serde(with = "ts_seconds")]
    pub published_at: DateTime<Utc>
}
