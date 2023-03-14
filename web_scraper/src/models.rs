use crate::schema::problems;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
pub struct Problems {
    pub id: i32,
    pub problem_name: String,
    pub url: String,
    pub has_rust: bool,
}

#[derive(Insertable)]
#[diesel(table_name = problems)]
pub struct NewProblem<'a> {
    pub problem_name: &'a str,
    pub url: &'a str,
    pub has_rust: &'a bool,
}
