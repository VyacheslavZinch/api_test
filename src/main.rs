mod auth;
mod db_requests;
mod models;
mod mongo;
mod s_fs;
mod schema;
use db_requests::Repos::{self, AircraftRepository, BoardingPass};
use diesel::result::Error::NotFound;
use futures::TryFutureExt;
use models::*;
use rocket::fs::{relative, FileServer};
use rocket::response::status;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{self, json, Json, Value},
};
use rocket_sync_db_pools::database;
use s_fs::server_fs::*;
use crate::auth::*;
use crate::db_requests::Repos::Seat;
use crate::models::NewBoardingPasses;

#[macro_use]
extern crate rocket;

#[database("postgres")]
struct DbConn(diesel::PgConnection);

#[get("/aircrafts")]
async fn get_aircrafts(_auth: TokenAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        AircraftRepository::get_all_datas(c, 100)
            .map(|aircrafts| json!(aircrafts))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/aircrafts/<id>")]
async fn view_aircraft(id: String, _auth: TokenAuth, db: DbConn) -> Result<Value, Custom<Value>> {
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
async fn get_boardpasses(_auth: TokenAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        BoardingPass::get_all_datas(c, 30000)
            .map(|boarding_passes| json!(boarding_passes))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/boardpasses/<id>")]
async fn get_boardpass(id: String, _auth: TokenAuth, db: DbConn) -> Result<Value, Custom<Value>> {
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

#[post("/boardpasses", format = "json", data = "<new_board_pass>")]
async fn create_boardpass(
    _auth: TokenAuth,
    db: DbConn,
    new_board_pass: Json<NewBoardingPasses>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        BoardingPass::create(c, new_board_pass.into_inner())
            .map(|boarding_passes| json!(boarding_passes))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/boardpasses/<id>")]
async fn del_boardpass(
    _auth: TokenAuth,
    db: DbConn,
    id: String,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        BoardingPass::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
#[get("/seats")]
async fn get_seats(_auth: TokenAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        Seat::get_all_datas(c, 1000)
            .map(|seats| json!(seats))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
#[post("/seats", format = "json", data = "<new_seat>")]
async fn create_new_seats(
    _auth: TokenAuth,
    db: DbConn,
    new_seat: Json<NewSeats>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        Seat::create(c, new_seat.into_inner())
            .map(|seats| json!(seats))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
#[get("/files")]
async fn get_files_info(_auth: TokenAuth) -> Json<Vec<FileInfo>> {
    let datas: Vec<FileInfo> = FileSystem::get_info_from_dir("files").await;
    Json(datas)
}

#[catch(404)]
fn error_404() -> Value {
    json!("NOT FOUND")
}
#[catch(401)]
fn error_401() -> Value {
    json!("INCORRECT TOKEN")
}

//TODO: Comments
//TODO: Tests
//TODO: PUT-requests
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(DbConn::fairing())
        .mount(
            "/",
            routes![
                get_aircrafts,
                view_aircraft,
                get_boardpasses,
                get_boardpass,
                create_boardpass,
                del_boardpass,
                get_seats,
                create_new_seats,
                get_files_info,
            ],
        )
        .mount("/file", FileServer::from(relative!("files")))
        .register("/", catchers![error_401, error_404])
        .launch()
        .await;
}
