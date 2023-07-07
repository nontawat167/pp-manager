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
