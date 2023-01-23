// @generated automatically by Diesel CLI.

diesel::table! {
    problems (id) {
        id -> Int4,
        problem_name -> Text,
        url -> Text,
        has_rust -> Bool,
    }
}
