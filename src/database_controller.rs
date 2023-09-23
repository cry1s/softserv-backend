use std::env;

use diesel::{prelude::*, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

use crate::models::Software;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new() -> Self {
        Self {
            connection: {
                let mut connection = establish_connection();
                connection.run_pending_migrations(MIGRATIONS).unwrap();
                connection
            },
        }
    }

    pub fn get_all_softwares(&mut self) -> Vec<Software> {
        use crate::schema::softwares::dsl::*;
        softwares
            .filter(active.eq(true))
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
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|e| {
        panic!(
            "Error connectionecting to {}, {}",
            database_url,
            e.to_string()
        )
    })
}
