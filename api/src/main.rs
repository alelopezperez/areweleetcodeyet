#[macro_use]
extern crate rocket;
use models::Problem;
use rocket::serde::json::Json;
mod leetcode_service;
mod models;
mod schema;
use crate::leetcode_service::*;

#[get("/all-problems")]
fn get_all_problems() -> Json<Vec<Problem>> {
    let conn = &mut establish_connection();
    Json(get_all(conn))
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![get_all_problems])
}
