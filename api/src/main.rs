#[macro_use]
extern crate rocket;
mod leetcode_service;
mod models;
mod schema;
use models::Problem;
use rocket::serde::json::Json;

#[get("/api/v1/all-problems")]
fn get_all_problems() -> Json<Vec<Problem>> {
    let conn = &mut leetcode_service::establish_connection();
    Json(leetcode_service::get_all(conn))
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![get_all_problems])
}
