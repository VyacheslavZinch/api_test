diesel::table! {
    aircrafts (aircraft_code) {
        aircraft_code -> Bpchar,
        model -> Text,
        range -> Integer,
    }
}

diesel::table! {
    seats (aircraft_code) {
        aircraft_code -> Bpchar,
        seat_no -> VarChar,
        fare_conditions -> VarChar
    }
}

diesel::table! {
    boarding_passes (ticket_no){
        ticket_no -> Bpchar,
        flight_id -> Integer,
        boarding_no -> Integer,
        seat_no -> VarChar
    }
}
