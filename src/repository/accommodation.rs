use uuid::Uuid;
use sqlx::error::Error;
use sqlx::postgres::PgPool;
use async_trait::async_trait;
use crate::repository::repository::Repository;
use crate::domain::accommodation::Accommodation;


pub struct AccommodationRepository {
    pool: PgPool,
}

impl AccommodationRepository {
    pub fn new(pool: &PgPool) -> Self {
        AccommodationRepository {
            pool: pool.clone(),
        }
    }
}

#[async_trait]
impl Repository<Accommodation> for AccommodationRepository {
    async fn insert(&self, payload: Accommodation) -> Result<Accommodation, Error> {
        let inserted: Accommodation = sqlx::query_as!(Accommodation, "INSERT INTO accommodations (id, hotel, guests, checkin, checkout, room)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, hotel, guests, checkin, checkout, room",
        payload.id,
        payload.hotel,
        payload.guests,
        payload.checkin,
        payload.checkout,
        payload.room)
        .fetch_one(&self.pool)
        .await?;

        Ok(inserted)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Accommodation>, Error> {
        // TODO: implementar usando sqlx
        match sqlx::query_as!(Accommodation,"SELECT * FROM accommodations WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(accommodation) => Ok(accommodation),
            Err(err) => Err(Error::from(err))
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        let query_result = sqlx::query!("DELETE FROM accommodations WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(Error::from)?;

        Ok(())
    }

    async fn patch(&self, id: Uuid, payload: Accommodation) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }
}
