use bigdecimal::BigDecimal;
use crate::utils::domainErrors::DomainError;


#[derive(Default, Debug)]
pub struct Travelinsurance {
    pub id: Option<i32>,
    pub client_id: i32,
    pub purpose_of_trip: String,
    pub luggage: Option<BigDecimal>,
    pub medical_cover: Option<BigDecimal>,
    pub price_total: BigDecimal,
}

pub fn new_insurance(client_id: i32, purpose_of_trip: String, luggage: Option<BigDecimal>, medical_cover: Option<BigDecimal>, price_total: BigDecimal) -> Result<Travelinsurance, DomainError> {

    let obj = Travelinsurance {
        id: None, // alterar o tipo para Option<String>, gerar UUID
        client_id,
        purpose_of_trip,
        luggage,
        medical_cover,
        price_total,
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
