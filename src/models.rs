// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use sqlx::prelude::*;
use uuid::Uuid;
use chrono::NaiveDate;
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Accommodation {
    pub id: Option<Uuid>,
    pub hotel: String,
    pub guests: i32,
    pub checkin: NaiveDate,
    pub checkout: Option<NaiveDate>,
    pub room: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub id: Option<Uuid>,
    pub name: String,
    pub address: Option<String>,
    pub occupation: Option<String>,
    pub birth_date: NaiveDate,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Eventticket {
    pub id: Option<Uuid>,
    pub client_id: Uuid,
    pub description: String,
    pub location: Option<String>,
    pub price: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guidedtour {
    pub id: Option<Uuid>,
    pub description: String,
    pub date: NaiveDate,
    pub participants: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Itinerary {
    pub id: Option<Uuid>,
    pub destination: String,
    pub departure: NaiveDate,
    pub arrival: Option<NaiveDate>,
    pub transport_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Travelinsurance {
    pub id: Option<Uuid>,
    pub client_id: Uuid,
    // #[sqlx(rename = "purposeOfTrip")]
    pub purpose_of_trip: String,
    pub luggage: Option<BigDecimal>,
    pub medical_cover: Option<BigDecimal>,
    pub price_total: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Travelpackage {
    pub id: Option<Uuid>,
    pub description: String,
    pub client_id: Uuid,
    pub country: String,
    pub city: String,
    pub accommodation_id: Option<Uuid>,
    pub insurance_id: Option<Uuid>,
    pub price_total: BigDecimal,
}
