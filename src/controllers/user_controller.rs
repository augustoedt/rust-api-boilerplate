use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::services::user_services::UserService;

#[derive(Deserialize, Serialize)]
pub struct UserIdData {
    pub id: String,
}

pub async fn get_users() -> impl Responder {
    let response = UserService::get_users();
    HttpResponse::Ok().json(response)
}

pub async fn delete_user(form: web::Form<UserIdData>) -> impl Responder {
    HttpResponse::Ok().body(format!("User id {{{}}} deleted.", form.id))
}
