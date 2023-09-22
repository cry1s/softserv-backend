use std::time::SystemTime;

use diesel::prelude::*;

use diesel_derive_enum::DbEnum;
use serde::Serialize;

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::RequestStatusEnum"]
pub enum RequestStatus {
    Created,
    Processed,
    Completed,
    Canceled,
    Deleted,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::SoftStatusEnum"]
pub enum SoftwareStatus {
    InQueue,
    Auto,
    Manual,
    Completed,
    Failed,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::connection_infos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConnectionInfo {
    id: i32,
    user_id: i32,
    ssh: String,
    password: String,
    valid: bool,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Request {
    id: i32,
    user_id: i32,
    moderator_id: Option<i32>,
    connection_info: i32,
    status: RequestStatus,
    created_at: SystemTime,
    updated_at: SystemTime,
    canceled_at: Option<SystemTime>,
    deleted_at: Option<SystemTime>,
    processed_at: Option<SystemTime>,
    completed_at: Option<SystemTime>,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::requests_softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RequestSoftware {
    id: i32,
    software_id: i32,
    request_id: i32,
    to_install: bool,
    port: i32,
    port_valid: Option<bool>,
    status: SoftwareStatus,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Software {
    id: i32,
    name: String,
    version: String,
    description: String,
    logo: Option<Vec<u8>>,
    source: String,
    active: bool,
    installation_script: Option<String>,
    deletion_script: Option<String>,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::softwares_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SoftwareTag {
    id: i32,
    software_id: i32,
    tag_id: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    id: i32,
    name: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    username: String,
    password: String,
    moderator: bool,
    created_at: SystemTime,
    updated_at: SystemTime,
}