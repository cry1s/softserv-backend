use crate::database_controller::Database;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use std::sync::Mutex;

#[derive(Deserialize)]
pub(crate) struct SoftwareFilter {
    pub search: Option<String>,
}

pub(crate) fn all_softwares(
    pool: web::Data<Mutex<Database>>,
    query: web::Query<SoftwareFilter>,
) -> impl Responder {
    let mut db = pool.lock().unwrap();
    let filter = query.into_inner();
    let response = db.get_all_active_softwares(filter);
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
pub(crate) struct SoftwareById {
    pub(crate) id: Option<String>,
}

pub(crate) async fn get_software(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<SoftwareById>,
) -> HttpResponse {
    if path.id.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен ID"
        }));
    }
    let id = path.id.take().unwrap().parse::<i32>();
    if id.is_err() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Неправильный ID"
        }));
    }
    let id = id.unwrap();
    let mut db = pool.lock().unwrap();
    let software = db.get_software_by_id(id);
    if software.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error": "ID не существует"
        }));
    }
    HttpResponse::Ok().json(json!({
        "software": software.unwrap()
    }))
}
