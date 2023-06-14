use sqlx::PgPool;
use actix_web::{post, web, HttpResponse, Responder};

use crate::library::schemas::{Book};
use crate::library::crud::{insert_book};


#[post("/book")]
pub async fn add_new_book(data: web::Json<Book>, pool: web::Data<PgPool>) -> impl Responder {
    match insert_book(&data, &pool).await {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => HttpResponse::InternalServerError()
    }
}
