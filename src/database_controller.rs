use std::env;
use std::time::SystemTime;

use diesel::{prelude::*, PgConnection, sql_query};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use serde::Deserialize;

use crate::{
    models::db_types::Tag,
    Software, schema::tags,
};
use crate::models::db_types::{Request, RequestStatus};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new() -> Self {
        Database::default()
    }

    pub fn get_all_active_softwares(&mut self) -> Vec<Software> {
        use crate::schema::softwares::dsl::*;
        softwares
            .filter(active.eq(true))
            .select(Software::as_select())
            .load(&mut self.connection)
            .unwrap()
    }

    pub fn get_softwares_by_name(&mut self, query: &str) -> Vec<Software> {
        use crate::schema::softwares::dsl::*;
        softwares
            .filter(name.ilike(format!("%{}%", query)).and(active.eq(true)))
            .select(Software::as_select())
            .load(&mut self.connection)
            .unwrap()
    }

    pub fn get_software_by_id(&mut self, id: i32) -> Option<Software> {
        use crate::schema::softwares::dsl::softwares;
        softwares
            .find(id)
            .get_result::<Software>(&mut self.connection)
            .ok()
    }

    pub fn get_tags_by_software(&mut self, soft_id: i32) -> Vec<Tag> {
        use crate::schema::softwares_tags::dsl::*;
        softwares_tags.filter(software_id.eq(soft_id))
        .inner_join(tags::table)
        .select(Tag::as_select())
        .load(&mut self.connection)
        .unwrap()
    }

    pub fn delete_software(&mut self, soft_id: i32) {
        sql_query(format!("UPDATE softwares SET active=false WHERE id={}", soft_id))
        .execute(&mut self.connection)
        .unwrap();
    }

    pub fn get_all_requests(&mut self, filter: RequestFilter) -> Vec<Request> {
        use crate::schema::requests::dsl::*;
        let mut query = requests.into_boxed();
        if let Some(filter_status) = filter.status {
            query = query.filter(status.eq(filter_status))
        }
        if let Some(create_date_start) = filter.create_date_start {
            if let Some(create_date_end) = filter.create_date_end {
                query = query.filter(created_at.between(create_date_start, create_date_end))
            } else {
                query = query.filter(created_at.ge(create_date_start))
            }
        } else if let Some(create_date_end) = filter.create_date_end {
            query = query.filter(created_at.le(create_date_end))
        }
        query.load(&mut self.connection).unwrap()
    }
}

#[derive(Deserialize)]
pub struct RequestFilter {
    pub status: Option<RequestStatus>,
    pub create_date_start: Option<SystemTime>,
    pub create_date_end: Option<SystemTime>
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
            let localhost = format!("{}@{}{}", &database_url[..a_sign], "localhost", &database_url[b_sign..]);
            PgConnection::establish(&localhost).unwrap()
        },
    }
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
