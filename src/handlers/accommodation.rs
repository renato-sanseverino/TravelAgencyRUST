use uuid::Uuid;
use sqlx::postgres::PgPool;
use actix_web::{get, post, web, HttpResponse, Error};
use crate::repository::repository::Repository;
use crate::domain::accommodation::Accommodation;
use crate::repository::accommodation::AccommodationRepository;


#[get("/hotels/{id}")]
async fn get_accommodation(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let rep = AccommodationRepository::new(&**pool);
    let accommodation = rep.get_by_id(id.into_inner())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(accommodation))
}

#[post("/hotels")]
async fn post_accommodation(pool: web::Data<PgPool>, payload: web::Json<Accommodation>) -> Result<HttpResponse, Error> {
    let accommodation: Accommodation = payload.into_inner();

    let rep = AccommodationRepository::new(&**pool);
    let inserted = rep.insert(accommodation)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(inserted))
}
