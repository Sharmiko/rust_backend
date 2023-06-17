use std::net::TcpListener;

use sqlx::PgPool;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;

use crate::library::routes::{add_new_book, list_books};


pub fn run(
    listener: TcpListener,
    connection_pool: PgPool
) -> Result<Server, std::io::Error> {

    let db_pool = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .service(add_new_book)
            .service(list_books)
    })
        .listen(listener)?
        .run();

    Ok(server)
}