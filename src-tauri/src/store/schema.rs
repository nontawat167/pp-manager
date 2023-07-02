// @generated automatically by Diesel CLI.

diesel::table! {
    skus (id) {
        id -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        name -> Text,
        price -> Integer,
        productType -> Text,
    }
}
