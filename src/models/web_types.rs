use std::sync::Mutex;

use actix_web::web::Data;
use serde::Serialize;

use base64::{engine::general_purpose, Engine as _};

use crate::database_controller::Database;

use super::db_types::{Software, Tag};

#[derive(Serialize)]
pub struct SoftwareCard {
    id: i32,
    logo: String,
    name: String,
    version: String,
    tags: Vec<Tag>,
    description: String,
}

impl SoftwareCard {
    pub fn new(
        db_soft: Software,
        pool: Data<Mutex<Database>>,
    ) -> Self {
        Self {
            id: db_soft.id,
            name: db_soft.name,
            version: db_soft.version,
            tags: {
                pool.lock().unwrap().get_tags_by_software(db_soft.id)
            },
            description: db_soft.description,
            logo: {
                if db_soft.logo.is_none() {
                    "/static/default_logo.png".to_string()
                } else {
                    let logo = db_soft.logo.unwrap();
                    format!("data:image/png;base64, {}", general_purpose::STANDARD.encode(&logo))
                }
            },
        }
    }
}
