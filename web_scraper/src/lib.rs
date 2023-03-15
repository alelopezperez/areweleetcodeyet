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
        url: url,
        has_rust,
    };

    diesel::insert_into(problems::table)
        .values(&new_post)
        .get_result(conn)
        .expect("msg")
}

pub fn get_last_problem(conn: &mut PgConnection) -> Problems {
    use schema::problems::dsl::*;

    let mut res = problems
        .limit(1)
        .order(id.desc())
        .load::<Problems>(conn)
        .expect("Error Fetching");

    res.swap_remove(0)
}

pub fn get_false_problems(conn: &mut PgConnection) -> Vec<Problems> {
    use schema::problems::dsl::*;

    let res = problems
        .filter(has_rust.eq(true))
        .load::<Problems>(conn)
        .expect("Error Fetching");

    res
}
