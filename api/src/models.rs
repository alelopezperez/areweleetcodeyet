use diesel::Queryable;
use rocket::serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Problem {
    pub id: Option<i32>,
    pub problem_name: String,
    pub url: String,
    pub has_rust: bool,
}
