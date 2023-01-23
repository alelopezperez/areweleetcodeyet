CREATE TABLE problems(
    id SERIAL PRIMARY KEY,
    problem_name TEXT NOT NULL,
    url TEXT NOT NULL,
    has_rust BOOLEAN NOT NULL
)