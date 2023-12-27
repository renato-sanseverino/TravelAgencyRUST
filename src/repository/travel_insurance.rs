use sqlx::error::Error;
use sqlx::postgres::PgPool;
use crate::repository::repository::Repository;
use crate::domain::travel_insurance::Travelinsurance;


pub struct InsuranceRepository {
    pool: PgPool,
    runtime: tokio::runtime::Runtime,
}

impl InsuranceRepository {
    pub fn new(pool: PgPool) -> Self {
        InsuranceRepository {
            pool: pool,
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

impl Repository<Travelinsurance> for InsuranceRepository {
    fn save(&mut self, payload: Travelinsurance) -> Result<(), Error> {
        // TODO: implementar usando sqlx e tokio
        Ok(())
    }

    fn get_by_id(&self, id: i32) -> Option<Travelinsurance> {
        let insurance = Travelinsurance::default();
        // TODO: implementar usando sqlx e tokio
        Some(insurance)
    }

    fn remove(&mut self, id: i32) -> Result<(), Error> {
        // TODO: implementar usando sqlx e tokio
        Ok(())
    }

    fn patch(&mut self, id: i32, payload: Travelinsurance) -> Result<(), Error> {
        // TODO: implementar usando sqlx e tokio
        Ok(())
    }
}
