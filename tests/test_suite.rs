use miso_backend::{configuration::DatabaseSettings, startup::run};
use names::Generator;
use std::net::TcpListener;

use miso_backend::configuration::source_configuration;
use sqlx::{Connection, Executor, MySqlConnection, MySqlPool};

struct TestApp {
    pub address: String,
    pub db_pool: MySqlPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_400_with_invalid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=joinemm&surname=boi", "missing email"),
        ("email=this.guy%40gmail.com", "missing name"),
        ("", "empty body"),
    ];
    for (body, error_message) in test_cases {
        let response = client
            .post(format!("http://{}/subscribe", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "Did not return 400 when {error_message}"
        );
    }
}
#[tokio::test]
async fn subscribe_returns_200_and_adds_data_to_table() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=joinemm&email=this.guy%40gmail.com";
    let response = client
        .post(format!("http://{}/subscribe", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // check status
    assert_eq!(200, response.status().as_u16());

    // check database content
    let data = sqlx::query!("SELECT email, name FROM subscription",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(data.email, "this.guy@gmail.com");
    assert_eq!(data.name, "joinemm");
}

async fn spawn_app() -> TestApp {
    let mut name_generator = Generator::default();
    let mut configuration = source_configuration().expect("Failed to read configuration");
    configuration.database.database_name = name_generator.next().unwrap();
    let connection_pool = configure_database(&configuration.database).await;
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, connection_pool.clone()).expect("Failed to run server");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("127.0.0.1:{port}"),
        db_pool: connection_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> MySqlPool {
    let mut connection = MySqlConnection::connect(&config.as_connection_string_without_db())
        .await
        .expect("Failed to connect to MySql");
    connection
        .execute(format!("CREATE DATABASE `{}`", config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = MySqlPool::connect(&config.as_connection_string())
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
