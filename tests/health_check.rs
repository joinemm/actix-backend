use std::net::TcpListener;

use miso_backend::startup::run;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_400_if_data_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=joinemm&surname=boi", "missing email"),
        ("email=this.guy%40gmail.com", "missing name"),
        ("", "empty body"),
    ];
    for (body, error_message) in test_cases {
        let response = client
            .post(format!("http://{address}/subscribe"))
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
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=joinemm&email=this.guy%40gmail.com";
    let response = client
        .post(format!("http://{address}/subscribe"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to run server");
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{port}")
}
