use std::net::TcpListener;

use sqlx::PgPool;
use secrecy::ExposeSecret;

use rust_backend::startup::run;
use rust_backend::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    let app_address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    );
    let listener = TcpListener::bind(app_address)
        .expect("Failed to bind port 80001.");

    run(listener, connection_pool)?.await
}
