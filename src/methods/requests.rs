use crate::database_controller::Database;
use crate::models::db_types::{InsertRequest, OptionInsertRequest, Request, RequestStatus, Software};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::time::SystemTime;

#[derive(Deserialize)]
pub(crate) struct RequestFilter {
    pub(crate) status: Option<RequestStatus>,
    pub(crate) create_date_start: Option<SystemTime>,
    pub(crate) create_date_end: Option<SystemTime>,
}

pub(crate) async fn get_all_requests(
    pool: web::Data<Mutex<Database>>,
    query: web::Query<RequestFilter>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let filter = query.into_inner();
    let response = db.get_all_requests(filter);
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
pub(crate) struct RequestById {
    pub(crate) id: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct RequestWithSoftwares {
    #[serde(flatten)]
    pub(crate) request: Request,
    pub(crate) softwares: Vec<Software>,
}

pub(crate) async fn get_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<RequestById>,
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
    let request = db.get_request(id);
    if request.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error": "ID не существует"
        }));
    }
    HttpResponse::Ok().json(json!({
        "request": request.unwrap()
    }))
}

#[derive(Deserialize)]
pub(crate) struct NewRequest {
    pub(crate) ssh_address: Option<String>,
    pub(crate) ssh_password: Option<String>,
}

pub(crate) async fn new_request(
    pool: web::Data<Mutex<Database>>,
    body: web::Json<NewRequest>,
) -> HttpResponse {
    let user_id = get_user_id_mock();
    let mut db = pool.lock().unwrap();
    let request = InsertRequest {
        user_id,
        ssh_address: body.0.ssh_address,
        ssh_password: body.0.ssh_password,
    };
    let response = db.new_request(request);
    match response {
        Ok(id) => {
            HttpResponse::Ok().json(json!({
                "id": id
            }))
        }
        Err(e) => {
            HttpResponse::BadRequest().json(json!({
                "error": e.to_string()
            }))
        }
    }
}

fn get_user_id_mock() -> i32 {
    1
}

pub(crate) async fn update_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<RequestById>,
    body: web::Json<OptionInsertRequest>,
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

    let mut db = pool.lock().unwrap();
    match db.update_request_by_id(id.unwrap(), body.into_inner()) {
        Ok(s) => {
            HttpResponse::Ok().json(s)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        }
    }
}