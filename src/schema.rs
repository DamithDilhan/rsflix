// @generated automatically by Diesel CLI.

diesel::table! {
    videos (ID) {
        ID -> Integer,
        title -> Text,
        description -> Text,
        removed -> Bool,
    }
}
