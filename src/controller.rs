use crate::methods::requests::RequestFilter;
use crate::methods::requests::RequestWithSoftwares;
use crate::methods::softwares::SoftwareFilter;
use crate::models::{AddImageSoftware, RequestStatus, SoftwareStatus};
use crate::models::{InsertRequest, OptionInsertRequest, OptionInsertSoftware, Tag};
use crate::models::{InsertSoftware, Request};
use crate::Software;
use diesel::{prelude::*, sql_query, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use serde_json::Value;
use serde_json::json;
use std::env;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub(crate) struct Database {
    connection: PgConnection,
}

impl Database {
    pub(crate) fn new() -> Self {
        Database::default()
    }

    pub(crate) fn is_moderator(&mut self, id: i32) -> bool {
        use crate::schema::users::dsl::*;
        let user = users.find(id)
            .get_result::<crate::models::User>(&mut self.connection)
            .ok();
        if user.is_none() {
            return false;
        };
        user.unwrap().moderator
    }

    pub(crate) fn get_all_active_softwares(&mut self, filter: SoftwareFilter) -> Value {
        use crate::schema::{softwares, softwares_tags, tags};
        let mut query = softwares::dsl::softwares.into_boxed();
        if let Some(search) = filter.search {
            let software_ids = softwares_tags::dsl::softwares_tags
                .inner_join(tags::dsl::tags.on(tags::id.eq(softwares_tags::tag_id)))
                .filter(tags::name.like(format!("%{}%", search)))
                .select(softwares_tags::software_id);

            use crate::schema::softwares::name;
            query = query.filter(
                name.like(format!("%{}%", search))
                    .or(softwares::dsl::id.eq_any(software_ids)),
            )
        };
        let softwares: Vec<Software> = query.load(&mut self.connection).unwrap();
        // applying tags
        let mut softwares_with_tags = vec![];
        for software in softwares {
            let tags = self.get_tags_by_software(software.id);
            softwares_with_tags.push(SoftwareWithTags {
                software,
                tags: tags.into_iter().map(|tag| tag.name).collect(),
            })
        }
        let user_id = get_user_id();
        let requestid = crate::schema::requests::dsl::requests
            .filter(
                crate::schema::requests::dsl::user_id.eq(user_id)
                    .and(
                        crate::schema::requests::dsl::status.eq(RequestStatus::Created)
                    )
            )
            .select(crate::schema::requests::dsl::id)
            .first::<i32>(&mut self.connection)
            .ok();
        json!({
            "softwares": softwares_with_tags,
            "request_id": requestid
        })
    }

    pub(crate) fn get_software_by_id(&mut self, id: i32) -> Option<SoftwareWithTags> {
        use crate::schema::softwares::dsl::softwares;
        let software: Software = softwares
            .find(id)
            .get_result::<Software>(&mut self.connection)
            .ok()?;

        let tags = self.get_tags_by_software(software.id);
        Some(SoftwareWithTags {
            software,
            tags: tags.into_iter().map(|tag| tag.name).collect(),
        })
    }

    pub(crate) fn get_tags_by_software(&mut self, soft_id: i32) -> Vec<Tag> {
        use crate::schema::softwares_tags::dsl::*;
        use crate::schema::tags;
        softwares_tags
            .filter(software_id.eq(soft_id))
            .inner_join(tags::table)
            .select(Tag::as_select())
            .load(&mut self.connection)
            .unwrap()
    }

    pub(crate) fn delete_software(&mut self, soft_id: i32) -> QueryResult<usize> {
        sql_query(format!(
            "UPDATE softwares SET active=false WHERE id={}",
            soft_id
        ))
        .execute(&mut self.connection)
    }

    pub(crate) fn get_all_requests(&mut self, filter: RequestFilter) -> Vec<RequestWithSoftwares> {
        use crate::schema::requests::dsl;
        let mut query = dsl::requests.into_boxed();
        if let Some(filter_status) = filter.status {
            query = query.filter(dsl::status.eq(filter_status))
        } else {
            query = query.filter(dsl::status.ne(RequestStatus::Deleted))
        }
        if let Some(create_date_start) = filter.create_date_start {
            let create_date_start = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(create_date_start as u64);
            if let Some(create_date_end) = filter.create_date_end {
                let create_date_end = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(create_date_end as u64);
                query = query.filter(dsl::created_at.between(create_date_start, create_date_end))
            } else {
                query = query.filter(dsl::created_at.ge(create_date_start))
            }
        } else if let Some(create_date_end) = filter.create_date_end {
            let create_date_end = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(create_date_end as u64);
            query = query.filter(dsl::created_at.le(create_date_end))
        }
        let requests: Vec<Request> = query.load(&mut self.connection).unwrap();
        let mut requests_with_softwares = vec![];
        for request in requests {
            let username = crate::schema::users::dsl::users
                .find(request.user_id)
                .select(crate::schema::users::dsl::username)
                .first::<String>(&mut self.connection)
                .unwrap();
            requests_with_softwares.push(RequestWithSoftwares {
                username,
                softwares: self.get_softwares_by_request(&request),
                request,
            })
        }
        requests_with_softwares
    }

    pub(crate) fn get_softwares_by_request(&mut self, request: &Request) -> Vec<Software> {
        use crate::schema::requests_softwares::dsl::*;
        use crate::schema::softwares;
        requests_softwares
            .filter(request_id.eq(request.id).and(status.ne(SoftwareStatus::Canceled)))
            .inner_join(softwares::table)
            .select(Software::as_select())
            .load(&mut self.connection)
            .unwrap_or(vec![])
    }

    pub(crate) fn get_request(&mut self, request_id: i32) -> Option<RequestWithSoftwares> {
        use crate::schema::requests::dsl::*;
        let request: Request = requests
            .find(request_id)
            .get_result::<Request>(&mut self.connection)
            .ok()?;
        Some(RequestWithSoftwares {
            username: {
                crate::schema::users::dsl::users
                    .find(request.user_id)
                    .select(crate::schema::users::dsl::username)
                    .first::<String>(&mut self.connection)
                    .unwrap()
            },
            softwares: self.get_softwares_by_request(&request),
            request,
        })
    }

    pub(crate) fn new_software(
        &mut self,
        name: String,
        active: bool,
        description: String,
        version: String,
        source: String,
    ) -> QueryResult<usize> {
        use crate::schema::softwares;
        InsertSoftware {
            description,
            active,
            name,
            version,
            source,
        }
        .insert_into(softwares::table)
        .execute(&mut self.connection)
    }

    pub(crate) fn update_software_by_id(&mut self, id: i32, new_data: OptionInsertSoftware) -> QueryResult<Software> {
        use crate::schema::softwares::dsl::softwares;
        diesel::update(
            softwares.find(id)
        ).set(
            new_data
        ).get_result::<Software>(&mut self.connection)
    }

    pub(crate) fn add_logo_to_software(&mut self, soft_id: i32, logo_url: &str) -> QueryResult<Software> {
        use crate::schema::softwares::dsl::softwares;
        diesel::update(
            softwares.find(soft_id)
        ).set(
            AddImageSoftware {
                logo: Some(logo_url.to_string()),
            }
        ).get_result::<Software>(&mut self.connection)
    }

    pub(crate) fn new_request(&mut self, request: InsertRequest) -> QueryResult<usize> {
        use crate::schema::requests;
        diesel::insert_into(requests::table)
            .values(request)
            .execute(&mut self.connection)
    }

    pub(crate) fn update_request_by_id(&mut self, id: i32, new_data: OptionInsertRequest) -> QueryResult<Request> {
        use crate::schema::requests::dsl::requests;
        diesel::update(
            requests.find(id)
        ).set(
            new_data
        ).get_result::<Request>(&mut self.connection)
    }

    pub(crate) fn get_tags_by_input(&mut self, input: String) -> Vec<Tag> {
        use crate::schema::tags::dsl::*;
        tags.filter(name.like(format!("%{}%", input)))
            .select(Tag::as_select())
            .load(&mut self.connection)
            .unwrap_or(vec![])
    }

    pub(crate) fn create_tag(&mut self, tag: String) -> QueryResult<()> {
        #[derive(Insertable)]
        #[table_name = "tags"]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct InsertTag {
            name: String,
        }

        use crate::schema::tags;
        InsertTag { name: tag }
            .insert_into(tags::table)
            .execute(&mut self.connection)
            .map(|_| ())
    }

    pub(crate) fn add_tag_to_software(&mut self, soft_id: i32, tag_id: i32) -> QueryResult<usize> {
        #[derive(Insertable)]
        #[table_name = "softwares_tags"]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct InsertTag {
            software_id: i32,
            tag_id: i32,
        }

        use crate::schema::softwares_tags;
        InsertTag { software_id: soft_id, tag_id }
            .insert_into(softwares_tags::table)
            .execute(&mut self.connection)
        
    }

    pub(crate) fn delete_tag_from_software(&mut self, soft_id: i32, tag_del_id: i32) -> QueryResult<usize> {
        use crate::schema::softwares_tags::dsl::*;
        diesel::delete(
            softwares_tags.filter(
                software_id.eq(soft_id)
                    .and(
                        tag_id.eq(tag_del_id)
                    )
            )
        ).execute(&mut self.connection)
    }

    pub(crate) fn add_software_to_last_request(&mut self, soft_id: i32, user_id: i32, to_install: bool) -> QueryResult<usize> {
        #[derive(Insertable)]
        #[table_name = "requests_softwares"]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct InsertRequestSoftware {
            software_id: i32,
            request_id: i32,
            to_install: bool,
            status: SoftwareStatus,
        }

        use crate::schema::requests_softwares;
        let request_id = crate::schema::requests::dsl::requests
            .filter(
                crate::schema::requests::dsl::user_id.eq(user_id)
                    .and(
                        crate::schema::requests::dsl::status.eq(RequestStatus::Created)
                    )
            )
            .order(crate::schema::requests::dsl::created_at.desc())
            .select(crate::schema::requests::dsl::id)
            .first::<i32>(&mut self.connection)?;

        InsertRequestSoftware {
            software_id: soft_id,
            request_id,
            to_install,
            status: SoftwareStatus::New,
        }.insert_into(requests_softwares::table)
            .execute(&mut self.connection)
    }

    // pub(crate) fn change_request_status(&mut self, request_id: i32, status: RequestStatus) -> QueryResult<usize> {
    //     use crate::schema::requests::dsl::*;
    //     diesel::update(
    //         requests.find(request_id)
    //     ).set(
    //         status.eq(status)
    //     ).execute(&mut self.connection)
    // }


    pub(crate) fn delete_request(&mut self, request_id: i32) -> QueryResult<usize> {
        use crate::schema::requests::dsl::*;
        diesel::update(
            requests.find(request_id)
        ).set(
            status.eq(RequestStatus::Deleted)
        ).execute(&mut self.connection)
    }

    pub(crate) fn add_software_to_request(&mut self, request_id: i32, soft_id: i32, to_install: bool) -> QueryResult<usize> {
        #[derive(Insertable)]
        #[table_name = "requests_softwares"]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct InsertRequestSoftware {
            software_id: i32,
            request_id: i32,
            to_install: bool,
            status: SoftwareStatus,
        }

        use crate::schema::requests_softwares;
        InsertRequestSoftware {
            software_id: soft_id,
            request_id,
            to_install,
            status: SoftwareStatus::New,
        }.insert_into(requests_softwares::table)
            .execute(&mut self.connection)
    }

    pub(crate) fn delete_software_from_request(&mut self, req_id: i32, soft_id: i32) -> QueryResult<usize> {
        use crate::schema::requests_softwares::dsl::*;
        diesel::update(
            requests_softwares.filter(
                request_id.eq(req_id)
                    .and(
                        software_id.eq(soft_id)
                    )
            )
        ).set(
            status.eq(SoftwareStatus::Canceled)
        ).execute(&mut self.connection)
    }

    pub(crate) fn change_request_software_status(&mut self, req_id: i32, soft_id: i32, new_status: SoftwareStatus) -> QueryResult<usize> {
        use crate::schema::requests_softwares::dsl::*;
        diesel::update(
            requests_softwares.filter(
                request_id.eq(req_id)
                    .and(
                        software_id.eq(soft_id)
                    )
            )
        ).set(
            status.eq(new_status)
        ).execute(&mut self.connection)
    }

    pub(crate) fn get_tag_by_id(&mut self, tag_id: i32) -> Option<Tag> {
        use crate::schema::tags::dsl::*;
        tags.find(tag_id)
            .get_result::<Tag>(&mut self.connection)
            .ok()
    }

    pub(crate) fn update_tag_by_id(&mut self, tag_id: i32, new_name: String) -> QueryResult<Tag> {
        use crate::schema::tags::dsl::*;
        diesel::update(
            tags.find(tag_id)
        ).set(
            name.eq(new_name)
        ).get_result::<Tag>(&mut self.connection)
    }

    pub(crate) fn apply_mod(&mut self, request_id: i32, mod_id: i32) -> QueryResult<usize> {
        use crate::schema::requests::dsl::*;
        diesel::update(
            requests.find(request_id)
        ).set(
            moderator_id.eq(mod_id)
        ).execute(&mut self.connection)
    }
}

fn get_user_id() -> i32 {
    1 // mock
}



fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgConnection::establish(&database_url) {
        Ok(conn) => conn,
        Err(e) => {
            // replacing hostname with localhost
            eprintln!("{}", e);
            let a_sign = database_url.find('@').unwrap();
            let b_sign = database_url[a_sign..].find('/').unwrap() + database_url[..a_sign].len();
            let localhost = format!(
                "{}@{}{}",
                &database_url[..a_sign],
                "localhost",
                &database_url[b_sign..]
            );
            PgConnection::establish(&localhost).unwrap()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SoftwareWithTags {
    pub(crate) software: Software,
    pub(crate) tags: Vec<String>,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            connection: {
                let mut connection = establish_connection();
                connection.run_pending_migrations(MIGRATIONS).unwrap();
                connection
            },
        }
    }
}
