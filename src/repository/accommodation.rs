use sqlx::error::Error;
use sqlx::postgres::PgPool;
use crate::repository::repository::Repository;
use crate::domain::accommodation::Accommodation;


pub struct AccommodationRepository {
    pool: PgPool,
    runtime: tokio::runtime::Runtime,
}

impl AccommodationRepository {
    pub fn new(pool: PgPool) -> Self {
        AccommodationRepository {
            pool: pool,
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

impl Repository<Accommodation> for AccommodationRepository {
    fn save(&mut self, payload: Accommodation) -> Result<(), Error> {
        // TODO: implementar usando sqlx e tokio
        Ok(())
    }

    fn get_by_id(&self, id: i32) -> Option<Accommodation> {
        let accommodation = Accommodation::default();
        // TODO: implementar usando sqlx e tokio
        Some(accommodation)
    }

    fn remove(&mut self, id: i32) -> Result<(), Error> {
        /*
            tokio::task::spawn(async move {
                let query_result = sqlx::query!("DELETE FROM accommodations WHERE id = $1", id)
                    .execute(&self.pool)
                    .await
                    .map_err(Error::from);

            });
        */
        Ok(())
    }

    fn patch(&mut self, id: i32, payload: Accommodation) -> Result<(), Error> {
        // TODO: implementar usando sqlx e tokio
        Ok(())
    }
}
