use uuid::Uuid;
use sqlx::PgPool;

use crate::library::schemas::Book;

pub async fn insert_book(data: &Book, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO books
            (id, author, title, pages, price, published_at)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        data.author,
        data.title,
        data.pages,
        data.price,
        data.published_at
    )
        .execute(pool)
        .await
        .map_err(|e| {
            e
        })?;

    Ok(())
}