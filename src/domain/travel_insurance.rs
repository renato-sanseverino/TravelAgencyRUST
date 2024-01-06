use uuid::Uuid;
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use crate::utils::domainErrors::DomainError;


#[derive(Serialize, Deserialize, Debug)]
pub struct Travelinsurance {
    pub id: Option<Uuid>,
    pub client_id: Uuid,
    pub purpose_of_trip: String,
//    pub luggage: Option<BigDecimal>,
//    pub medical_cover: Option<BigDecimal>,    
//    pub price_total: BigDecimal,
}

pub fn new_insurance(client_id: Uuid, purpose_of_trip: String, luggage: Option<BigDecimal>, medical_cover: Option<BigDecimal>, price_total: BigDecimal) -> Result<Travelinsurance, DomainError> {

    let obj = Travelinsurance {
        id: Some(Uuid::new_v4()),
        client_id,
        purpose_of_trip,
//        luggage,
//        medical_cover,
//        price_total,
    };

    if !obj.validate() {
        return Err(DomainError::ValidationError);
    }
    Ok(obj)
}

impl Travelinsurance {
    pub fn validate(&self) -> bool {
        
        true
    }
}
