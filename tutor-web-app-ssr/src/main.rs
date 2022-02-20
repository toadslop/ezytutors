#[path = "./mod.rs"]
mod modules;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use modules::{dbaccess, errors, handlers, models, routes, state};
use routes::{app_config, course_config};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::env;
use tera::Tera;
use crate::handlers::general::not_found;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_port = env::var("HOST_PORT")
        .expect("HOST_PORT address is not set in .env file");
    println!("Listening on {}!", &host_port);

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(AppState { db: db_pool});

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/templates/**/*"))
            .unwrap();

        App::new()
            .data(tera)
            .app_data(shared_data.clone())
            .configure(course_config)
            .configure(app_config)
            .default_service(
                web::route().to(not_found)
            )
    })
    .bind(&host_port)?
    .run()
    .await
}
