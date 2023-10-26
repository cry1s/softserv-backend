use std::time::SystemTime;

use diesel::prelude::*;

use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::RequestStatusEnum"]
pub(crate) enum RequestStatus {
    Created,
    Processed,
    Completed,
    Canceled,
    Deleted,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::SoftStatusEnum"]
pub(crate) enum SoftwareStatus {
    New,
    Processed,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Selectable, Queryable, Serialize)]
#[diesel(table_name = crate::schema::requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Request {
    pub(crate) id: i32,
    pub(crate) user_id: i32,
    pub(crate) moderator_id: Option<i32>,
    pub(crate) status: RequestStatus,
    pub(crate) ssh_address: Option<String>,
    pub(crate) ssh_password: Option<String>,
    pub(crate) created_at: SystemTime,
    pub(crate) processed_at: Option<SystemTime>,
    pub(crate) completed_at: Option<SystemTime>,
}

#[derive(Identifiable, Debug, Selectable, Queryable, Associations)]
#[diesel(primary_key(software_id, request_id))]
#[diesel(belongs_to(Software))]
#[diesel(belongs_to(Request))]
#[diesel(table_name = crate::schema::requests_softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct RequestSoftware {
    software_id: i32,
    request_id: i32,
    to_install: bool,
    status: SoftwareStatus,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Identifiable, Debug, Selectable, Queryable, Serialize)]
#[diesel(table_name = crate::schema::softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Software {
    pub(crate) id: i32,
    pub(crate) description: String,
    pub(crate) logo: Option<String>,
    pub(crate) active: bool,
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) source: String,
    pub(crate) created_at: SystemTime,
    pub(crate) updated_at: SystemTime,
}

#[derive(Identifiable, Debug, Selectable, Queryable, Associations)]
#[diesel(primary_key(software_id, tag_id))]
#[diesel(belongs_to(Software))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = crate::schema::softwares_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct SoftwareTag {
    software_id: i32,
    tag_id: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Identifiable, Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Tag {
    id: i32,
    name: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}

#[derive(Serialize, Debug, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct User {
    id: i32,
    username: String,
    password: String,
    moderator: bool,
    created_at: SystemTime,
    updated_at: SystemTime,
}
