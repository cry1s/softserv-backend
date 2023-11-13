use crate::controller::Database;
use crate::models::{InsertRequest, OptionInsertRequest, Request, RequestStatus, Software, SoftwareStatus};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::time::SystemTime;
use super::Response;

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

#[derive(Deserialize)]
pub(crate) struct AddSoftwareToLastRequestPayload {
    pub(crate) software_id: i32,
    pub(crate) to_install: bool
}
pub(crate) async fn add_software_to_last_request(
    pool: web::Data<Mutex<Database>>,
    payload: web::Json<AddSoftwareToLastRequestPayload>,
) -> HttpResponse {
    let user_id = get_user_id_mock();
    let mut db = pool.lock().unwrap();
    let response = db.add_software_to_last_request(payload.software_id, user_id, payload.to_install);
    response.response(json!({
        "status": "ok"
    }))
}

#[derive(Deserialize)]
pub(crate) struct ChangeRequestStatusPayload {
    pub(crate) status: Option<RequestStatus>,
}

pub(crate) async fn change_request_status(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<RequestById>,
    body: web::Json<ChangeRequestStatusPayload>,
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
    if body.status.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Не представлен status"
        }));
    }
    let _user_id = get_user_id_mock();
    // TODO: check if user is moderator
    let mut db = pool.lock().unwrap();
    let response = db.change_request_status(id.unwrap(), body.status.unwrap());
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn delete_request(
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

    let mut db = pool.lock().unwrap();
    let response = db.delete_request(id.unwrap());
    response.response(json!({
        "status": "ok"
    }))
}

#[derive(Deserialize)]
pub(crate) struct AddSoftwareToRequestPayload {
    pub(crate) software_id: Option<i32>,
    pub(crate) to_install: Option<bool>
}

pub(crate) async fn add_software_to_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<(Option<i32>,)>,
    payload: web::Json<AddSoftwareToRequestPayload>,
) -> HttpResponse {
    if path.0.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен request_id"
        }));
    }
    if payload.software_id.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен software_id"
        }));
    }
    let request_id = path.0.take().unwrap();
    let mut db = pool.lock().unwrap();
    let response = db.add_software_to_request(request_id, payload.software_id.unwrap(), payload.to_install.unwrap_or(true));
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn delete_software_from_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<(Option<i32>, Option<i32>)>,
) -> HttpResponse {
    if path.0.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен request_id"
        }));
    }
    if path.1.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен software_id"
        }));
    }
    let request_id = path.0.take().unwrap();
    let software_id = path.1.take().unwrap();
    let mut db = pool.lock().unwrap();

    let _user_id = get_user_id_mock();
    // TODO check if user is moderator and owner of request

    let response = db.delete_software_from_request(request_id, software_id);
    response.response(json!({
        "status": "ok"
    }))
}

#[derive(Deserialize)]
pub(crate) struct ChangeRequestSoftwareStatusPayload {
    pub(crate) status: Option<SoftwareStatus>,
}

pub(crate) async fn change_request_software_status(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<(Option<i32>, Option<i32>)>,
    body: web::Json<ChangeRequestSoftwareStatusPayload>,
) -> HttpResponse {
    if path.0.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен request_id"
        }));
    }
    if path.1.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен software_id"
        }));
    }
    if body.status.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен status"
        }));
    }
    let request_id = path.0.take().unwrap();
    let software_id = path.1.take().unwrap();
    let mut db = pool.lock().unwrap();

    let _user_id = get_user_id_mock();
    // TODO check if user is moderator and owner of request

    let response = db.change_request_software_status(request_id, software_id, body.status.unwrap());
    response.response(json!({
        "status": "ok"
    }))
}
