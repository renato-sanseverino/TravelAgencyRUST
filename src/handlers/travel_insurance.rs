use uuid::Uuid;
use sqlx::postgres::PgPool;
use actix_web::{get, post, web, HttpResponse, Error};
use crate::repository::repository::Repository;
use crate::domain::travel_insurance::Travelinsurance;
use crate::repository::travel_insurance::InsuranceRepository;


#[get("/insurances/{id}")]
async fn get_insurance(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let rep = InsuranceRepository::new(&**pool);
    let insurance = rep.get_by_id(id.into_inner())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(insurance))
}

#[post("/insurances")]
async fn post_insurance(pool: web::Data<PgPool>, payload: web::Json<Travelinsurance>) -> Result<HttpResponse, Error> {
    let insurance: Travelinsurance = payload.into_inner();

    let rep = InsuranceRepository::new(&**pool);
    let inserted = rep.insert(insurance)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(inserted))
}
