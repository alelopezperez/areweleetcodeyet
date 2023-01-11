// @generated automatically by Diesel CLI.

diesel::table! {
    problems (id) {
        id -> Nullable<Integer>,
        problem_name -> Text,
        url -> Text,
        has_rust -> Bool,
    }
}
