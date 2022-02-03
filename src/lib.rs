use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/{id}/{name}/index.html")]
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = path.into_inner();
    format!("Hello {}! id:{}", name, id)
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(index).service(health_check))
        .listen(listener)?
        .run();

    Ok(server)
}
