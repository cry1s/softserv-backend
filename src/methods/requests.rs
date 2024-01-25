use super::Response;
use crate::controller::Database;
use crate::models::{InsertRequest, OptionInsertRequest, Request, RequestStatus, Software, TokenClaims};
use actix_web::web::ReqData;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;
use std::time::SystemTime;

#[derive(Deserialize)]
pub(crate) struct RequestFilter {
    pub(crate) status: Option<RequestStatus>,
    pub(crate) create_date_start: Option<i32>,
    pub(crate) create_date_end: Option<i32>,
}

pub(crate) async fn get_all_requests(
    pool: web::Data<Mutex<Database>>,
    query: web::Query<RequestFilter>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    let claims = claims.unwrap();
    let mut db = pool.lock().unwrap();
    let filter = query.into_inner();
    let uid = if claims.moderator { None } else { Some(claims.uid) };
    let response = db.get_all_requests(filter, uid);
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
pub(crate) struct RequestById {
    pub(crate) id: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct RequestWithSoftwares {
    pub(crate) softwares: Vec<Software>,
    pub(crate) request: Request,
    pub(crate) username: String,
}

pub(crate) async fn get_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<RequestById>,
    claims: Option<ReqData<TokenClaims>>
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
    let request = request.unwrap();
    let claims = claims.unwrap();
    if request.request.user_id != claims.uid && !claims.moderator {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно прав"
        }));
    }
    HttpResponse::Ok().json(json!({
        "request": request
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
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    let user_id = claims.unwrap().uid;
    let mut db = pool.lock().unwrap();
    let request = InsertRequest {
        user_id,
        ssh_address: body.0.ssh_address,
        ssh_password: body.0.ssh_password,
    };
    let response = db.new_request(request);
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn update_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<RequestById>,
    mut body: web::Json<OptionInsertRequest>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    if !claims.unwrap().moderator && body.mod_rights_needed() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно прав"
        }));
    }
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

    match body.0.status {
        Some(RequestStatus::Completed) => {
            body.0.completed_at = Some(SystemTime::now());
        }
        Some(RequestStatus::Processed) => {
            body.0.processed_at = Some(SystemTime::now());
        }
        _ => {}
    }

    let mut db = pool.lock().unwrap();
    match db.update_request_by_id(id.unwrap(), body.into_inner()) {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

#[derive(Deserialize)]
pub(crate) struct AddSoftwareToLastRequestPayload {
    pub(crate) software_id: i32,
}
pub(crate) async fn add_software_to_last_request(
    pool: web::Data<Mutex<Database>>,
    payload: web::Json<AddSoftwareToLastRequestPayload>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    println!("{:?}", claims);
    let user_id = claims.unwrap().uid;
    let mut db = pool.lock().unwrap();
    let response = db.add_software_to_last_request(payload.software_id, user_id);
    match response {
        Ok(s) => HttpResponse::Ok().json(json!({
            "request_id": s
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

#[derive(Deserialize)]
pub(crate) struct ChangeRequestStatusPayload {
    pub(crate) status: Option<RequestStatus>,
}

pub(crate) async fn change_request_status(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<RequestById>,
    body: web::Json<ChangeRequestStatusPayload>,
    claims: Option<ReqData<TokenClaims>>
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

    let mut db = pool.lock().unwrap();
    let is_moderator = claims.unwrap().moderator;

    if !is_moderator
        && body.status.unwrap() != RequestStatus::Processed
            && body.status.unwrap() != RequestStatus::Canceled
    {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно прав"
        }));
    }
    let id = id.unwrap();
    let new_status = body.status.unwrap();
    let current_status = db.get_request(id).unwrap().request.status;
    let mut upd = OptionInsertRequest::default();

    match (current_status, new_status) {
        (RequestStatus::Created, RequestStatus::Processed) => {
            upd.status = Some(RequestStatus::Processed);
            upd.processed_at = Some(SystemTime::now());
        }
        (RequestStatus::Processed, RequestStatus::Completed) => {
            upd.status = Some(RequestStatus::Completed);
            upd.completed_at = Some(SystemTime::now());
        }
        (cur, RequestStatus::Canceled)
            if cur != RequestStatus::Deleted || cur != RequestStatus::Created =>
        {
            upd.status = Some(RequestStatus::Canceled);
        }
        _ => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Неправильный переход"
            }));
        }
    }

    upd.status = body.status;
    let response = db.update_request_by_id(id, upd);
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn change_request_status_admin(
    pool: web::Data<Mutex<Database>>,
    path: web::Path<RequestById>,
    body: web::Json<ChangeRequestStatusPayload>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    change_request_status(pool, path, body, claims).await
}

pub(crate) async fn delete_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<RequestById>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    if !claims.unwrap().moderator {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно прав"
        }));
    }
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
}

pub(crate) async fn add_software_to_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<(Option<i32>,)>,
    payload: web::Json<AddSoftwareToRequestPayload>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    if !claims.unwrap().moderator {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно прав. Пожалуйста, используйте POST /request/add"
        }));
    }
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
    let response = db.add_software_to_request(request_id, payload.software_id.unwrap());
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn delete_software_from_request(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<(Option<i32>, Option<i32>)>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    if claims.unwrap().moderator {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Недостаточно прав"
        }));
    }
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

    let response = db.delete_software_from_request(request_id, software_id);
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn apply_mod(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<(Option<i32>,)>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    let claims = claims.unwrap();
    if claims.moderator {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Недостаточно прав"
        }));
    }
    if path.0.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен request_id"
        }));
    }
    let request_id = path.0.take().unwrap();
    let mut db = pool.lock().unwrap();
    let user_id = claims.uid;

    let response = db.apply_mod(request_id, user_id);
    response.response(json!({
        "status": "ok"
    }))
}
