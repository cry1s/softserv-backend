use std::env;

use diesel::{PgConnection, Connection, prelude::*};
use dotenvy::dotenv;

use crate::models::Software;

pub struct Database {
    conn: PgConnection,
}

impl Database {
    pub fn new() -> Self {
        Self {
            conn: establish_connection(),
        }
    }

    pub fn get_all_softwares(&mut self) -> Vec<Software> {
        use crate::schema::softwares::dsl::*;
        softwares
            .filter(active.eq(true))
            .select(Software::as_select())
            .load(&mut self.conn)
            .unwrap()
    }

    pub fn get_software_by_id(&mut self, id: i32) -> Option<Software> {
        use crate::schema::softwares::dsl::softwares;
        softwares
            .find(id)
            .get_result::<Software>(&mut self.conn)
            .ok()
        
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
