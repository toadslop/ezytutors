use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::general::not_found;
use std::env;
use std::io;
use std::sync::Mutex;
use sqlx::postgres::PgPoolOptions;

#[path = "./app/dbaccess/mod.rs"]
mod dbaccess;

#[path = "./app/errors.rs"]
mod errors;

#[path = "./app/handlers/mod.rs"]
mod handlers;

#[path = "./app/models/mod.rs"]
mod models;

#[path = "./app/routes.rs"]
mod routes;

#[path = "./app/state.rs"]
mod state;

use routes::*;
use errors::EzyTutorError;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                EzyTutorError::InvalidInput("Please provide valid JSON input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
            .default_service(
                web::route().to(not_found)
            )
    };

    let host_port = env::var("HOST_PORT").expect("HOST_PORT address is not set in .env file");
    HttpServer::new(app)
        .bind(&host_port)?.run().await
}