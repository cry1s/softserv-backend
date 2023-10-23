use std::sync::Mutex;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use crate::database_controller::Database;

#[derive(Deserialize)]
pub struct SoftwareFilter {
    pub search: Option<String>
}

async fn all_softwares(
    pool: web::Data<Mutex<Database>>,
    query: web::Query<SoftwareFilter>,
) -> impl Responder {
    let mut db = pool.lock().unwrap();
    let filter = query.into_inner();
    let response = db.get_all_active_softwares(filter);
    HttpResponse::Ok().json(response)
}