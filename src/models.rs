use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::aircrafts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Aircraft {
    pub aircraft_code: String,
    pub model: String,
    pub range: i32,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::aircrafts)]
pub struct NewAircraft {
    pub aircraft_code: String,
    pub model: String,
    pub range: i32,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::seats)]
pub struct NewSeats {
    pub aircraft_code: String,
    pub seat_no: String,
    pub fare_conditions: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::seats)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Seats {
    pub aircraft_code: String,
    pub seat_no: String,
    pub fare_conditions: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::boarding_passes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BoardingPasses {
    pub ticket_no: String,
    pub flight_id: i32,
    pub boarding_no: i32,
    pub seat_no: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::boarding_passes)]
pub struct NewBoardingPasses {
    pub ticket_no: String,
    pub flight_id: i32,
    pub boarding_no: i32,
    pub seat_no: String,
}
