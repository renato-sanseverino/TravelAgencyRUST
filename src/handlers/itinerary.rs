use uuid::Uuid;
use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/itineraries")]
async fn index(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let itineraries: Vec<Itinerary> = sqlx::query_as!(Itinerary,"SELECT * FROM itinerary")
    .fetch_all(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(itineraries))
}

#[post("/itineraries")]
async fn create(pool: web::Data<PgPool>, payload: web::Json<Itinerary>) -> Result<HttpResponse, Error> {
    let itinerary_payload: Itinerary = payload.into_inner();

    let inserted_itinerary: Itinerary = sqlx::query_as!(Itinerary, "INSERT INTO itinerary(destination, departure, arrival, transport_kind)
    VALUES ($1, $2, $3, $4)
    RETURNING id, destination, departure, arrival, transport_kind",
    itinerary_payload.destination,
    itinerary_payload.departure,
    itinerary_payload.arrival,
    itinerary_payload.transport_kind)
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(inserted_itinerary))
}

#[get("/itineraries/{itinerary_id}")]
async fn select(pool: web::Data<PgPool>, itinerary_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as!(Itinerary, "SELECT * FROM itinerary WHERE id = $1", itinerary_id.into_inner())
    .fetch_optional(&**pool)
    .await;

    match result {
        Ok(Some(itinerary)) => Ok(HttpResponse::Ok().json(itinerary)),
        Ok(None) => Ok(HttpResponse::NotFound().body("Itinerary not found")),
        Err(e) => {
            eprintln!("Error querying database: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[patch("/itineraries/{itinerary_id}")]
async fn update(pool: web::Data<PgPool>, itinerary_id: web::Path<Uuid>, payload: web::Json<Itinerary>) -> Result<HttpResponse, Error> {
    let itinerary_payload: Itinerary = payload.into_inner();

    let updated_itinerary: Itinerary = sqlx::query_as!(Itinerary, "UPDATE itinerary
    SET (destination, departure, arrival, transport_kind) = ($2, $3, $4, $5)
    WHERE id = $1
    RETURNING id, destination, departure, arrival, transport_kind",
    itinerary_id.into_inner(),
    itinerary_payload.destination,
    itinerary_payload.departure,
    itinerary_payload.arrival,
    itinerary_payload.transport_kind)
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(updated_itinerary))
}

#[delete("/itineraries/{itinerary_id}")]
async fn delete(pool: web::Data<PgPool>, itinerary_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let query_result = sqlx::query!("DELETE FROM itinerary WHERE id = $1", itinerary_id.into_inner())
        .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows_affected: u64 = query_result.rows_affected();
    Ok(HttpResponse::Ok().json(rows_affected))
}
