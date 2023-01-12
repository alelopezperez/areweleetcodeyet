use crate::models::Problem;
use crate::schema::problems::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("COnnection Error")
}

pub fn get_all(conn: &mut SqliteConnection) -> Vec<Problem> {
    let res = problems.load::<Problem>(conn).expect("ERRO GETIN PROBLEMS");

    res
}
