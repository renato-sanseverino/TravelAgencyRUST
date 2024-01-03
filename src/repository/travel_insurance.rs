use sqlx::error::Error;
use sqlx::postgres::PgPool;
use async_trait::async_trait;
use crate::repository::repository::Repository;
use crate::domain::travel_insurance::Travelinsurance;


pub struct InsuranceRepository {
    pool: PgPool,
}

impl InsuranceRepository {
    pub fn new(pool: PgPool) -> Self {
        InsuranceRepository {
            pool: pool,
        }
    }
}

#[async_trait]
impl Repository<Travelinsurance> for InsuranceRepository {
    async fn insert(&mut self, payload: Travelinsurance) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Travelinsurance>, Error> {
        let insurance = Travelinsurance::default();
        // TODO: implementar usando sqlx
        Ok(Some(insurance))
    }

    async fn delete(&mut self, id: i32) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }

    async fn patch(&mut self, id: i32, payload: Travelinsurance) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }
}
