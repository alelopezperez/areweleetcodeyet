-- Your SQL goes here
CREATE TABLE problems(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    problem_name TEXT NOT NULL,
    url TEXT NOT NULL,
    has_rust BOOLEAN NOT NULL
)