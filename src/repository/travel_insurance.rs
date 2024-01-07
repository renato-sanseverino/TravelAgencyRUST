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
        let inserted = sqlx::query_as!(Travelinsurance, "INSERT INTO travelinsurance (id, client_id, \"purposeOfTrip\", luggage, medical_cover, price_total)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, client_id, \"purposeOfTrip\" as purpose_of_trip, luggage, medical_cover, price_total",
        payload.id,
        payload.client_id,
        payload.purpose_of_trip,
        payload.luggage,
        payload.medical_cover,
        payload.price_total)
        .fetch_one(&self.pool)
        .await?;

        Ok(inserted)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Travelinsurance>, Error> {
        match sqlx::query_as!(Travelinsurance,"SELECT id, client_id, \"purposeOfTrip\" as purpose_of_trip, luggage, medical_cover, price_total FROM travelinsurance WHERE id = $1", id)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(insurance) => Ok(insurance),
            Err(err) => Err(Error::from(err))
        }
    }

    async fn delete(&self, id: Uuid) -> Result<u64, Error> {
        let query_result = sqlx::query!("DELETE FROM travelinsurance WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(Error::from)?;

        Ok(query_result.rows_affected())
    }

    async fn patch(&self, _id: Uuid, _payload: Travelinsurance) -> Result<(), Error> {
        // TODO: implementar usando sqlx
        Ok(())
    }
}
