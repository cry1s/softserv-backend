use std::time::SystemTime;

use diesel::prelude::*;

use serde::Serialize;

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::connection_infos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConnectionInfo {
    id: i32,
    ssh: String,
    password: String,
    valid: bool,
}

#[derive(Serialize, Debug, Selectable, Queryable, Default)]
#[diesel(table_name = crate::schema::softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Software {
    id: i32,
    name: String,
    version: String,
    description: String,
    logo: Option<Vec<u8>>,
    status: String,
    source: String,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Request {
    id: i32,
    user_id: i32,
    moderator_id: Option<i32>,
    program_id: i32,
    connection_info: i32,
    status: String,
    created_at: SystemTime,
    updated_at: SystemTime,
    canceled_at: Option<SystemTime>,
    deleted_at: Option<SystemTime>,
    processed_at: Option<SystemTime>,
    completed_at: Option<SystemTime>,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    id: i32,
    name: String,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    username: String,
    password: String,
    moderator: bool,
}