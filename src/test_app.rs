use std::net::TcpListener;

use uuid::Uuid;
use secrecy::ExposeSecret;
use sqlx::{PgConnection, Connection, PgPool, Executor};

use crate::startup::run;
use crate::configuration::{get_configuration, DatabaseSettings};


pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}


impl TestApp {
    pub async fn spawn_app() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.db_name = Uuid::new_v4().to_string();
        let connection_pool = Self::configure_database(&configuration.database).await;

        let server = run(listener, connection_pool.clone())
            .expect("Failed to bind address");

        let _ = tokio::spawn(server);

        TestApp {
            address,
            db_pool: connection_pool
        }
    }

    pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
        let mut connection = PgConnection::connect(&config.connection_string_without_db().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

        connection.execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
            .await
            .expect("Failed to create database");

        let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate database");

        connection_pool
    }
}
