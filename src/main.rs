use env_logger::Env;
use miso_backend::configuration::source_configuration;
use miso_backend::startup::run;
use sqlx::MySqlPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = source_configuration().expect("Failed to read configuration");
    let connection = MySqlPool::connect(&configuration.database.as_connection_string())
        .await
        .expect("Failed to connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
