use uuid::Uuid;
use sqlx::error::Error;
use sqlx::postgres::PgPool;
use async_trait::async_trait;
use crate::repository::repository::Repository;
use crate::domain::travel_insurance::Travelinsurance;


pub struct InsuranceRepository {
    pool: PgPool,
}

impl InsuranceRepository {
    pub fn new(pool: &PgPool) -> Self {
        InsuranceRepository {
            pool: pool.clone(),
        }
    }
}

#[async_trait]
impl Repository<Travelinsurance> for InsuranceRepository {
    async fn insert(&self, payload: Travelinsurance) -> Result<Travelinsurance, Error> {
        // TODO: implementar usando sqlx
        Ok(Travelinsurance::default())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Travelinsurance>, Error> {
        // TODO: implementar usando sqlx
        Ok(Some(Travelinsurance::default()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }

    async fn patch(&self, id: Uuid, payload: Travelinsurance) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }
}
