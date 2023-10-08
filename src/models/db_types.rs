use std::time::SystemTime;

use diesel::prelude::*;

use diesel_derive_enum::DbEnum;
use serde::Serialize;

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::RequestStatusEnum"]
pub enum RequestStatus {
    Created,
    Processed,
    Completed,
    Canceled,
    Deleted,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::SoftStatusEnum"]
pub enum SoftwareStatus {
    InQueue,
    Auto,
    Manual,
    Completed,
    Failed,
}

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Request {
    id: i32,
    user_id: i32,
    moderator_id: Option<i32>,
    status: RequestStatus,
    created_at: SystemTime,
    processed_at: Option<SystemTime>,
    completed_at: Option<SystemTime>,
}

#[derive(Identifiable, Debug, Selectable, Queryable, Associations)]
#[diesel(primary_key(software_id, request_id))]
#[diesel(belongs_to(Software))]
#[diesel(belongs_to(Request))]
#[diesel(table_name = crate::schema::requests_softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RequestSoftware {
    software_id: i32,
    request_id: i32,
    to_install: bool,
    status: SoftwareStatus,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Identifiable, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Software {
    pub id: i32,
    pub description: String,
    pub logo: Option<String>,
    pub active: bool,
    pub name: String,
    pub version: String,
    pub source: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Identifiable, Debug, Selectable, Queryable, Associations)]
#[diesel(primary_key(software_id, tag_id))]
#[diesel(belongs_to(Software))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = crate::schema::softwares_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SoftwareTag {
    software_id: i32,
    tag_id: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Identifiable, Serialize, Debug, Selectable, Queryable)]
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
