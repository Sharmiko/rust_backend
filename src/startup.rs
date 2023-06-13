use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;


pub fn run(
    listener: TcpListener
) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(move || {
        App::new()
    })
        .listen(listener)?
        .run();

    Ok(server)
}