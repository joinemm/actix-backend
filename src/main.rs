use miso_backend::configuration::source_configuration;
use miso_backend::startup::run;
use sqlx::MySqlPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = source_configuration().expect("Failed to read configuration");
    let connection = MySqlPool::connect(&configuration.database.as_connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
