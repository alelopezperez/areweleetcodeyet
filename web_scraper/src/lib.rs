pub mod models;
pub mod schema;

use self::models::{NewProblem, Problems};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::problems::problem_name;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_problem(conn: &mut PgConnection, p: &str, url: &str, has_rust: &bool) -> Problems {
    use crate::schema::problems;

    let new_post = NewProblem {
        problem_name: p,
        url,
        has_rust,
    };

    diesel::insert_into(problems::table)
        .values(&new_post)
        .get_result(conn)
        .expect("msg")
}
