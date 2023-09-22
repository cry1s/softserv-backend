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
    connection_infos (id) {
        id -> Int4,
        user_id -> Int4,
        ssh -> Varchar,
        password -> Varchar,
        valid -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RequestStatusEnum;

    requests (id) {
        id -> Int4,
        user_id -> Int4,
        moderator_id -> Nullable<Int4>,
        connection_info -> Int4,
        status -> RequestStatusEnum,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        canceled_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        processed_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SoftStatusEnum;

    requests_softwares (id) {
        id -> Int4,
        software_id -> Int4,
        request_id -> Int4,
        to_install -> Bool,
        port -> Int4,
        port_valid -> Nullable<Bool>,
        status -> SoftStatusEnum,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    softwares (id) {
        id -> Int4,
        name -> Varchar,
        version -> Varchar,
        description -> Text,
        logo -> Nullable<Bytea>,
        source -> Varchar,
        active -> Bool,
        installation_script -> Nullable<Text>,
        deletion_script -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    softwares_tags (id) {
        id -> Int4,
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
        moderator -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(requests -> connection_infos (connection_info));
diesel::joinable!(requests_softwares -> requests (request_id));
diesel::joinable!(requests_softwares -> softwares (software_id));
diesel::joinable!(softwares_tags -> softwares (software_id));
diesel::joinable!(softwares_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    connection_infos,
    requests,
    requests_softwares,
    softwares,
    softwares_tags,
    tags,
    users,
);
