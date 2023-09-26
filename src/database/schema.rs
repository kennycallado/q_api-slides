// @generated automatically by Diesel CLI.

diesel::table! {
    media (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        media_type -> Varchar,
        url -> Text,
    }
}

diesel::table! {
    slides (id) {
        id -> Int4,
        slide_type -> Varchar,
        title -> Varchar,
        media_id -> Nullable<Int4>,
        content -> Nullable<Text>,
        question_id -> Nullable<Int4>,
    }
}

diesel::joinable!(slides -> media (media_id));

diesel::allow_tables_to_appear_in_same_query!(
    media,
    slides,
);
