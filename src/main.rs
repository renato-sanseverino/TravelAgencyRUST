mod utils;
mod schema;
mod models;
mod domain;
mod repository;
mod handlers;


use handlers::client;
use handlers::itinerary;
use actix_cors::Cors;
use actix_web::{web, middleware, App, HttpServer};
// use diesel::prelude::*;                       // diesel ORM
use sqlx::postgres::{PgPool, PgPoolOptions};     // sqlx


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().expect("Unable to load environment variables from .env file");
    let database_url: String = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool_options = PgPoolOptions::new().max_connections(100);
    let pool: PgPool = pool_options.connect(&database_url)
        .await
        .expect("Unable to connect to database");

    HttpServer::new(move || {
        let cors: Cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(
                web::scope("/api")
                    .service(client::index)
                    .service(client::create)
                    .service(client::select)
                    .service(client::update)
                    .service(client::delete)
                    .service(itinerary::index)
                    .service(itinerary::create)
                    .service(itinerary::select)
                    .service(itinerary::update)
                    .service(itinerary::delete)
                    .service(handlers::travel_insurance::get_insurance)
                    .service(handlers::accommodation::get_accommodation)
                    .service(handlers::accommodation::post_accommodation)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
