use chrono::NaiveDate;
use bigdecimal::BigDecimal;
use crate::utils::domainErrors::DomainError;


#[derive(PartialEq)]
pub struct Charge {
    value: BigDecimal,
}

pub struct Accommodation {
    pub id: Option<i32>,
    pub hotel: String,
    pub guests: i32,
    pub checkin: NaiveDate,
    pub checkout: Option<NaiveDate>,
    pub room: Option<i32>,
    pub charges: Vec<Charge>,
}

impl Accommodation {
    pub fn new(hotel: String, guests: i32, checkin: NaiveDate, checkout: Option<NaiveDate>, room: Option<i32>) -> Result<Accommodation, DomainError> {
        let obj = Accommodation {
            id: None, // alterar o tipo para Option<String>, gerar UUID
            hotel,
            guests,
            checkin,
            checkout,
            room,
            charges: Vec::new(),
        };

        Ok(obj)
    }

    pub fn add_charge(&mut self, charge: Charge) -> Result<(), DomainError>{
        if self.charges.contains(&charge) {
            return Err(DomainError::DuplicationError)
        }
        self.charges.push(charge);
        Ok(())
    }

    pub fn remove_charge(&mut self, index: usize) {
        if index < self.charges.len() {
            self.charges.remove(index);
        }
    }

    pub fn validate(&self) -> bool {
        
        // Verifica se a data de checkout é posterior a data de checkin
        if let Some(checkout_date) = self.checkout {
            if checkout_date <= self.checkin {
                return false;
            }
        }
        true
    }
}
