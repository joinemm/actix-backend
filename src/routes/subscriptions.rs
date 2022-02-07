use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::MySqlPool;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(form: web::Form<FormData>, connection: web::Data<MySqlPool>) -> impl Responder {
    match sqlx::query!(
        "INSERT INTO subscription (name, email, subscribed_at) VALUES (?, ?, ?)",
        form.name,
        form.email,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}
