use crate::models::Problem;
use crate::schema::problems::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("COnnection Error")
}

pub fn get_all(conn: &mut PgConnection) -> Vec<Problem> {
    let res = problems.load::<Problem>(conn).expect("ERRO GETIN PROBLEMS");

    res
}
