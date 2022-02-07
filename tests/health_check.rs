use std::net::TcpListener;

use miso_backend::startup::run;

use miso_backend::configuration::source_configuration;
use sqlx::MySqlPool;

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
async fn subscribe_400_if_data_missing() {
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
async fn subscribe_200_with_valid_data() {
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
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let configuration = source_configuration().expect("Failed to read configuration");
    let connection_pool = MySqlPool::connect(&configuration.database.as_connection_string())
        .await
        .expect("Failed to connect to database");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, connection_pool.clone()).expect("Failed to run server");
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("127.0.0.1:{port}"),
        db_pool: connection_pool,
    }
}
