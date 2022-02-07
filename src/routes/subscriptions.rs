use actix_web::{ post, web, HttpResponse,  Responder};

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    print!("{0} {1}", form.name, form.email);
    HttpResponse::Ok()
}