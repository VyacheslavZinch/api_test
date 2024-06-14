mod auth;
mod auth1;
mod db_requests;
mod models;
mod mongo;
mod schema;

use db_requests::Repos::{AircraftRepository, BoardingPass};
use diesel::result::Error::NotFound;

use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{json, Value},
};
use rocket_sync_db_pools::database;
use crate::auth1::TokenAuth;

use crate::auth::BasicAuth;

#[macro_use]
extern crate rocket;

#[database("postgres")]
struct DbConn(diesel::PgConnection);

#[get("/aircrafts")]
async fn get_aircrafts(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        AircraftRepository::get_all_datas(c, 100)
            .map(|aircrafts| json!(aircrafts))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/aircrafts/<id>")]
async fn view_aircraft(id: String, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        AircraftRepository::find(c, id)
            .map(|aircrafts| json!(aircrafts))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
    })
    .await
}

#[get("/boardpasses")]
async fn get_bpasses(_auth: TokenAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        BoardingPass::get_all_datas(c, 30000)
            .map(|boarding_passes| json!(boarding_passes))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/boardpasses/<id>")]
async fn get_bpass(id: String, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        BoardingPass::find(c, id)
            .map(|boarding_passes| json!(boarding_passes))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
    })
    .await
}

#[catch(404)]
fn not_found_404() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(DbConn::fairing())
        .mount(
            "/",
            routes![get_aircrafts, view_aircraft, get_bpasses, get_bpass],
        )
        .register("/", catchers![not_found_404])
        .launch()
        .await;
}
