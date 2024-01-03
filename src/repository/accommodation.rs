use sqlx::error::Error;
use sqlx::postgres::PgPool;
use async_trait::async_trait;
use crate::repository::repository::Repository;
use crate::domain::accommodation::Accommodation;


pub struct AccommodationRepository {
    pool: PgPool,
}

impl AccommodationRepository {
    pub fn new(pool: PgPool) -> Self {
        AccommodationRepository {
            pool: pool,
        }
    }
}

#[async_trait]
impl Repository<Accommodation> for AccommodationRepository {
    async fn insert(&mut self, payload: Accommodation) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Accommodation>, Error> {
        // TODO: implementar usando sqlx
        let accommodation = Accommodation::default();
        /*
        match sqlx::query_as!(Accommodation,"SELECT id, hotel, guests, checkin, checkout, room, charges as charges FROM accommodations WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(accommodation) => Ok(accommodation),
            Err(err) => Err(Error::from(err))
        }
        */
        Ok(Some(accommodation))
    }

    async fn delete(&mut self, id: i32) -> Result<(), Error> {
        let query_result = sqlx::query!("DELETE FROM accommodations WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(Error::from)?;

        Ok(())
    }

    async fn patch(&mut self, id: i32, payload: Accommodation) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }
}
