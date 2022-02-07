use crate::routes::*;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::MySqlPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: MySqlPool) -> std::io::Result<Server> {
    let connection_arc = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .app_data(connection_arc.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
