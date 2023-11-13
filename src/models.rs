
use std::time::SystemTime;



use diesel::prelude::*;


use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::RequestStatusEnum"]
#[DbValueStyle = "snake_case"]
pub(crate) enum RequestStatus {
    Created,
    Processed,
    Completed,
    Canceled,
    Deleted,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::schema::sql_types::SoftStatusEnum"]
#[DbValueStyle = "snake_case"]
pub(crate) enum SoftwareStatus {
    New,
    Processed,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Selectable, Queryable, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

#[derive(Insertable, Debug, Selectable, Queryable, Serialize)]
#[diesel(table_name = crate::schema::requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct InsertRequest {
    pub(crate) user_id: i32,
    pub(crate) ssh_address: Option<String>,
    pub(crate) ssh_password: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct OptionInsertRequest {
    pub(crate) ssh_address: Option<String>,
    pub(crate) ssh_password: Option<String>,
    pub(crate) moderator_id: Option<i32>,
    pub(crate) status: Option<RequestStatus>,
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

#[derive(Insertable, Debug, Selectable, Queryable, Serialize)]
#[diesel(table_name = crate::schema::softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct InsertSoftware {
    pub(crate) description: String,
    pub(crate) active: bool,
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) source: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::softwares)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct OptionInsertSoftware {
    pub(crate) description: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) active: Option<bool>,
    pub(crate) name: Option<String>,
    pub(crate) source: Option<String>,
}

impl OptionInsertSoftware {
    pub(crate) fn any_none(&self) -> bool {
        self.description.is_none() || self.active.is_none() || self.name.is_none() || self.version.is_none() || self.source.is_none()
    }

    pub(crate) fn all_none(&self) -> bool {
        self.description.is_none() && self.active.is_none() && self.name.is_none() && self.version.is_none() && self.source.is_none()
    }
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
