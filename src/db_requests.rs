pub mod Repos {
    use diesel::prelude::*;
    use diesel::{PgConnection, QueryDsl, QueryResult};

    use crate::{
        models::*,
        schema::{self, *},
    };
    pub struct AircraftRepository;

    impl AircraftRepository {
        pub fn find(c: &mut PgConnection, id: String) -> QueryResult<Aircraft> {
            aircrafts::table.find(id).get_result::<Aircraft>(c)
        }

        pub fn get_all_datas(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Aircraft>> {
            aircrafts::table
                .order(aircrafts::aircraft_code.desc())
                .limit(limit)
                .load::<Aircraft>(c)
        }

        pub fn create(c: &mut PgConnection, new_aircraft: NewAircraft) -> QueryResult<Aircraft> {
            diesel::insert_into(aircrafts::table)
                .values(new_aircraft)
                .execute(c)?;

            let last_id = Self::last_inserted_id(c)?;
            Self::find(c, last_id)
        }

        fn last_inserted_id(c: &mut PgConnection) -> QueryResult<String> {
            aircrafts::table
                .select(aircrafts::aircraft_code)
                .order(aircrafts::aircraft_code.desc())
                .first(c)
        }

        pub fn save(c: &mut PgConnection, id: String, aircraft: Aircraft) -> QueryResult<Aircraft> {
            diesel::update(aircrafts::table.find(id.clone()))
                .set((
                    aircrafts::aircraft_code.eq(aircraft.aircraft_code.to_owned()),
                    aircrafts::range.eq(aircraft.range.to_owned()),
                    aircrafts::model.eq(aircraft.model.to_owned()),
                ))
                .execute(c)?;

            Self::find(c, id)
        }

        pub fn delete(c: &mut PgConnection, id: String) -> QueryResult<usize> {
            diesel::delete(aircrafts::table.find(id)).execute(c)
        }
    }

    pub struct BoardingPass;

    impl BoardingPass {
        pub fn find(c: &mut PgConnection, id: String) -> QueryResult<BoardingPasses> {
            boarding_passes::table
                .find(id)
                .get_result::<BoardingPasses>(c)
        }

        pub fn get_all_datas(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<BoardingPasses>> {
            boarding_passes::table
                .order(boarding_passes::ticket_no.desc())
                .limit(limit)
                .load::<BoardingPasses>(c)
        }

        pub fn create(
            c: &mut PgConnection,
            new_aircraft: NewBoardingPasses,
        ) -> QueryResult<BoardingPasses> {
            diesel::insert_into(boarding_passes::table)
                .values(new_aircraft)
                .execute(c)?;

            let last_id = Self::last_inserted_id(c)?;
            Self::find(c, last_id)
        }

        fn last_inserted_id(c: &mut PgConnection) -> QueryResult<String> {
            boarding_passes::table
                .select(boarding_passes::ticket_no)
                .order(boarding_passes::ticket_no.desc())
                .first(c)
        }

        pub fn save(
            c: &mut PgConnection,
            id: String,
            bpass: BoardingPasses,
        ) -> QueryResult<BoardingPasses> {
            diesel::update(boarding_passes::table.find(id.clone()))
                .set((
                    boarding_passes::ticket_no.eq(bpass.ticket_no.to_owned()),
                    boarding_passes::flight_id.eq(bpass.flight_id.to_owned()),
                    boarding_passes::boarding_no.eq(bpass.boarding_no.to_owned()),
                    boarding_passes::seat_no.eq(bpass.seat_no.to_owned()),
                ))
                .execute(c)?;

            Self::find(c, id)
        }

        pub fn delete(c: &mut PgConnection, id: String) -> QueryResult<usize> {
            diesel::delete(boarding_passes::table.find(id)).execute(c)
        }
    }
}
