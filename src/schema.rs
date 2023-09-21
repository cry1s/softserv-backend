use diesel::{table, allow_tables_to_appear_in_same_query};

table! {
    connection_infos (id) {
        id -> Integer,
        user_id -> Integer,
        ssh -> Varchar,
        password -> Varchar,
        valid -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    softwares (id) {
        id -> Integer,
        name -> Varchar,
        version -> Varchar,
        description -> Text,
        logo -> Nullable<Binary>,
        source -> Varchar,
        status -> VarChar, 
        installation_script -> Nullable<Text>,
        deletion_script -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    softwares_tags (id) {
        id -> Integer,
        software_id -> Integer,
        tag_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    requests (id) {
        id -> Integer,
        user_id -> Integer,
        moderator_id -> Nullable<Integer>,
        program_id -> Integer,
        connection_info -> Integer,
        status -> VarChar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        canceled_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        processed_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
    }
}

table! {
    requests_softwares (id) {
        id -> Integer,
        software_id -> Integer,
        port -> Integer,
        port_valid -> Nullable<Bool>,
        status -> VarChar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        moderator -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    connection_infos,
    softwares,
    softwares_tags,
    requests,
    requests_softwares,
    tags,
    users,
);