// @generated automatically by Diesel CLI.

diesel::table! {
    skus (id) {
        id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        name -> Text,
        price -> Integer,
        product_type -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Text,
        kind -> Text,
        color -> Text,
    }
}

diesel::joinable!(skus -> tags (product_type));

diesel::allow_tables_to_appear_in_same_query!(
    skus,
    tags,
);
