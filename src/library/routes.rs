use sqlx::PgPool;
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::library::schemas::{Book};
use crate::library::crud;


#[post("/book")]
pub async fn add_new_book(data: web::Json<Book>, pool: web::Data<PgPool>) -> impl Responder {
    match crud::insert_book(&data, &pool).await {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => HttpResponse::InternalServerError()
    }
}

#[get("/book")]
pub async fn list_books(pool: web::Data<PgPool>) -> impl Responder {
    match crud::list_books(&pool).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().finish()
    }
}