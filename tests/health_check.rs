use std::net::TcpListener;

use miso_backend::run;

#[tokio::test]
async fn health_check_works() {
    let port = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://127.0.0.1:{0}/health_check", port))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to run server");
    let _ = tokio::spawn(server);
    port
}
