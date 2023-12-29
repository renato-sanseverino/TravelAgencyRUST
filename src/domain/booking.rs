use crate::utils::domainErrors::DomainError;
// use crate::domain::accommodation::Accommodation;


pub fn book_a_room(room_number: i32, booked_rooms: &Vec<i32>) -> Result<Vec<i32>, DomainError> {

    // if (available_rooms < 1) {
    //    return Err(DomainError::NotEnoughError);
    // }

    if booked_rooms.contains(&room_number) {
        return Err(DomainError::NotAvailableError);
    }

    let mut booked_rooms_copy: Vec<i32> = booked_rooms.clone();
    booked_rooms_copy.push(room_number);

    Ok(booked_rooms_copy)
}
