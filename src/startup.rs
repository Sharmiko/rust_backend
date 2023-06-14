use std::net::TcpListener;

use sqlx::PgPool;

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;



pub fn run(
    listener: TcpListener,
    connection_pool: PgPool
) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection_pool.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}