use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use std::env;

mod controllers;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("API_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("API_PORT");

    println!("Starting server on http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/users").configure(routes::user_routes::init_routes))
    })
    .bind((host, port))?
    .run()
    .await
}
