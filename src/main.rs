use std::net::TcpListener;

use miso_backend::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    run(listener)?.await
}
