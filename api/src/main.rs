#[macro_use]
extern crate rocket;
mod leetcode_service;
mod models;
mod schema;
use models::Problem;
use rocket::serde::json::Json;
#[macro_use]
extern crate diesel;

#[get("/api/v1/all-problems")]
fn get_all_problems() -> Json<Vec<Problem>> {
    let conn = &mut leetcode_service::establish_connection();
    Json(leetcode_service::get_all(conn))
}
#[get("/api/v1/hello")]
fn get_hello() -> Json<String> {
    //let conn = &mut leetcode_service::establish_connection();
    Json("HOLA".to_string())
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![get_all_problems, get_hello])
}
