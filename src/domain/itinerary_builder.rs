use reqwest::*;
use chrono::NaiveDate;
// use crate::utils::domainErrors::DomainError;
// use crate::domain::transportation::Transport;

pub struct Transport {

}

// Verifica a disponibilidade de voos na API de terceiros
pub async fn consultar_disponibilidade(transport_kind: Transport, date: NaiveDate) -> Result<()>{
    let request_url: String = String::from("https:// ...  /api/ConsultaAereo/Consultar");

    let request_obj = reqwest::Client::new().get(request_url);
    let response = request_obj.send().await.unwrap();

    if !response.status().is_success() {
        return Err(response.error_for_status().unwrap_err());
    }
    Ok(())
}
