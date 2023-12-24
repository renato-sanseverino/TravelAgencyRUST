use crate::models::*;
// use diesel::prelude::*;      // diesel ORM
use sqlx::postgres::PgPool;     // sqlx
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/clients")]
async fn index(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let clients: Vec<Client> = sqlx::query_as!(Client, "SELECT * FROM client")
    .fetch_all(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(clients))
}

#[post("/clients")]
async fn create(pool: web::Data<PgPool>, payload: web::Json<Client>) -> Result<HttpResponse, Error> {
    let client_payload: Client = payload.into_inner();

    let inserted_client: Client = sqlx::query_as!(Client, "INSERT INTO client(name, address, occupation, birth_date, email)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING id, name, address, occupation, birth_date, email",
    client_payload.name,
    client_payload.address,
    client_payload.occupation,
    client_payload.birth_date,
    client_payload.email)
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(inserted_client))
}

#[get("/clients/{client_id}")]
async fn select(pool: web::Data<PgPool>, client_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let result = sqlx::query_as!(Client, "SELECT * FROM client WHERE id = $1", client_id.into_inner())
    .fetch_optional(&**pool)
    .await;

    match result {
        Ok(Some(client)) => Ok(HttpResponse::Ok().json(client)),
        Ok(None) => Ok(HttpResponse::NotFound().body("Client not found")),
        Err(e) => {
            eprintln!("Error querying database: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[patch("/clients/{client_id}")]
async fn update(pool: web::Data<PgPool>, client_id: web::Path<i32>, payload: web::Json<Client>) -> Result<HttpResponse, Error> {
    let client_payload: Client = payload.into_inner();

    let updated_client: Client = sqlx::query_as!(Client, "UPDATE client
    SET (name, address, occupation, birth_date, email) = ($2, $3, $4, $5, $6)
    WHERE id = $1
    RETURNING id, name, address, occupation, birth_date, email",
    client_id.into_inner(),
    client_payload.name,
    client_payload.address,
    client_payload.occupation,
    client_payload.birth_date,
    client_payload.email)
    .fetch_one(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(updated_client))
}

#[delete("/clients/{client_id}")]
async fn delete(pool: web::Data<PgPool>, client_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let query_result = sqlx::query!("DELETE FROM client WHERE id = $1", client_id.into_inner())
        .execute(&**pool)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows_affected: u64 = query_result.rows_affected();
    Ok(HttpResponse::Ok().json(rows_affected))
}
