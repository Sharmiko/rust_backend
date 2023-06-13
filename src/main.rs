use std::net::TcpListener;

use rust_backend::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8001")
        .expect("Failed to bind port 80001.");

    run(listener)?.await
}
