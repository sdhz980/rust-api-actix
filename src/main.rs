mod config;
mod errors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use std::env;
use actix_web::{ middleware::Logger, App, HttpServer };
use sqlx::migrate;
use dotenv::dotenv;

use crate::config::database::create_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = create_pool(&database_url).await;

    migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    println!("SERVER STARTING ON HTTP://{}:{}", host, port);

    HttpServer::new(move || {
        App::new().wrap(Logger::default()).wrap(middleware::cors::cors()).configure(routes::config)
    })
        .bind(format!("{}:{}", host, port))?
        .run().await
}
