// @generated automatically by Diesel CLI.

diesel::table! {
    slides (id) {
        id -> Int4,
        slide_type -> Varchar,
        title -> Varchar,
        content -> Nullable<Text>,
        question_id -> Nullable<Int4>,
    }
}
