// @generated automatically by Diesel CLI.

/*
diesel::table! {
    accommodations (id) {
        id -> Int4,
        #[max_length = 120]
        hotel -> Varchar,
        guests -> Int4,
        checkin -> Date,
        checkout -> Nullable<Date>,
        room -> Nullable<Int4>,
    }
}

diesel::table! {
    client (id) {
        id -> Int4,
        #[max_length = 120]
        name -> Varchar,
        #[max_length = 200]
        address -> Nullable<Varchar>,
        #[max_length = 120]
        occupation -> Nullable<Varchar>,
        birth_date -> Date,
        #[max_length = 80]
        email -> Varchar,
    }
}

diesel::table! {
    eventtickets (id) {
        id -> Int4,
        client_id -> Int4,
        #[max_length = 80]
        description -> Varchar,
        #[max_length = 80]
        location -> Nullable<Varchar>,
        price -> Numeric,
    }
}

diesel::table! {
    guidedtours (id) {
        id -> Int4,
        #[max_length = 120]
        description -> Varchar,
        date -> Date,
        participants -> Int4,
    }
}

diesel::table! {
    itinerary (id) {
        id -> Int4,
        #[max_length = 120]
        destination -> Varchar,
        departure -> Date,
        arrival -> Nullable<Date>,
        #[max_length = 65]
        transport_kind -> Varchar,
    }
}

diesel::table! {
    travelinsurance (id) {
        id -> Int4,
        client_id -> Int4,
        #[max_length = 80]
        purposeOfTrip -> Varchar,
        luggage -> Nullable<Numeric>,
        medical_cover -> Nullable<Numeric>,
        price_total -> Numeric,
    }
}

diesel::table! {
    travelpackages (id) {
        id -> Int4,
        #[max_length = 120]
        description -> Varchar,
        client_id -> Int4,
        #[max_length = 80]
        country -> Varchar,
        #[max_length = 80]
        city -> Varchar,
        accommodation_id -> Nullable<Int4>,
        insurance_id -> Nullable<Int4>,
        price_total -> Numeric,
    }
}

diesel::joinable!(travelpackages -> accommodations (accommodation_id));
diesel::joinable!(travelpackages -> client (client_id));
diesel::joinable!(travelpackages -> travelinsurance (insurance_id));

diesel::allow_tables_to_appear_in_same_query!(
    accommodations,
    client,
    eventtickets,
    guidedtours,
    itinerary,
    travelinsurance,
    travelpackages,
);
*/
