use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FormData {
    pub message: String,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Rota GET!")
}

pub async fn post_route(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Rota POST! Mensagem: {}", form.message))
}
