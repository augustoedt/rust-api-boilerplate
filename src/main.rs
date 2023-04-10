use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("API_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("API_PORT deve ser um número");

    println!("Iniciando servidor em http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(controllers::user_controller::get_users))
            .route(
                "/post",
                web::post().to(controllers::user_controller::delete_user),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
