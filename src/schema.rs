// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "request_status_enum"))]
    pub struct RequestStatusEnum;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "soft_status_enum"))]
    pub struct SoftStatusEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RequestStatusEnum;

    requests (id) {
        id -> Int4,
        user_id -> Int4,
        moderator_id -> Nullable<Int4>,
        status -> RequestStatusEnum,
        ssh_address -> Nullable<Varchar>,
        ssh_password -> Nullable<Varchar>,
        created_at -> Timestamp,
        processed_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SoftStatusEnum;

    requests_softwares (software_id, request_id) {
        software_id -> Int4,
        request_id -> Int4,
        to_install -> Bool,
        status -> SoftStatusEnum,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    softwares (id) {
        id -> Int4,
        description -> Text,
        logo -> Nullable<Varchar>,
        active -> Bool,
        name -> Varchar,
        version -> Varchar,
        source -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    softwares_tags (software_id, tag_id) {
        software_id -> Int4,
        tag_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        avatar -> Varchar,
        moderator -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(requests_softwares -> requests (request_id));
diesel::joinable!(requests_softwares -> softwares (software_id));
diesel::joinable!(softwares_tags -> softwares (software_id));
diesel::joinable!(softwares_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    requests,
    requests_softwares,
    softwares,
    softwares_tags,
    tags,
    users,
);
