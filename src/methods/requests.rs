use std::sync::Mutex;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use serde_json::json;
use crate::database_controller::Database;
use crate::models::db_types::{Request, RequestStatus, Software};

#[derive(Deserialize)]
pub struct RequestFilter {
    pub status: Option<RequestStatus>,
    pub create_date_start: Option<SystemTime>,
    pub create_date_end: Option<SystemTime>
}

pub async fn get_all_requests(
    pool: web::Data<Mutex<Database>>,
    query: web::Query<RequestFilter>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let filter = query.into_inner();
    let response = db.get_all_requests(filter);
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
pub struct RequestById {
    id: Option<String>
}

#[derive(Serialize)]
pub struct RequestWithSoftwares {
    #[serde(flatten)]
    pub request: Request,
    pub softwares: Vec<Software>
}

pub async fn get_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<RequestById>
) -> HttpResponse {
    if path.id.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен ID"
        }))
    }
    let id = path.id.take().unwrap().parse::<i32>();
    if id.is_err() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Неправильный iD"
        }))
    }
    let id = id.unwrap();
    let db = pool.lock().unwrap().get_request(id);
    HttpResponse::Ok().finish()
}