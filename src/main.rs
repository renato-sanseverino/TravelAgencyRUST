mod utils;
mod schema;
mod models;
mod domain;
mod repository;
mod handlers;


use chrono::NaiveDate;
use handlers::client;
use handlers::itinerary;
use domain::accommodation::*;
use domain::travel_insurance::*;
use actix_cors::Cors;
use actix_web::{web, middleware, App, HttpServer};
// use diesel::prelude::*;                       // diesel ORM
use sqlx::postgres::{PgPool, PgPoolOptions};     // sqlx


fn check_domain() {
    let accommodation = new_accommodation(
        String::from("hotel"), 4, NaiveDate::MIN, Some(NaiveDate::MAX), Some(12)
    );
    println!("{:?}", accommodation);

    // Limitar o uso de default, pois a entidade de domínio não é validada
    let insurance = Travelinsurance::default();
    println!("{:?}", insurance);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().expect("Unable to load environment variables from .env file");
    let database_url: String = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
    check_domain();

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
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
