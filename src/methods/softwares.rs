use crate::database_controller::Database;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use std::sync::Mutex;
use crate::models::db_types::OptionInsertSoftware;

#[derive(Deserialize)]
pub(crate) struct SoftwareFilter {
    pub search: Option<String>,
}

pub(crate) async fn all_softwares(
    pool: web::Data<Mutex<Database>>,
    query: web::Query<SoftwareFilter>,
) -> HttpResponse {
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

pub(crate) async fn update_software(
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<SoftwareById>,
    body: web::Json<OptionInsertSoftware>
) -> HttpResponse {
    if path.id.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен ID"
        }));
    }

    if body.all_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Пустое тело"
        }));
    }

    let id = path.id.take().unwrap().parse::<i32>();
    if id.is_err() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Неправильный ID"
        }));
    }

    let mut db = pool.lock().unwrap();
    match db.update_software_by_id(id.unwrap(), body.into_inner()) {
        Ok(s) => {
            HttpResponse::Ok().json(s)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            }))
        },
    }
}

pub(crate) async fn new_software(
    pool: web::Data<Mutex<Database>>,
    mut body: web::Json<OptionInsertSoftware>
) -> HttpResponse {
    if body.any_none() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно полей"
        }));
    }

    let mut db = pool.lock().unwrap();
    match db.new_software(
        body.0.name.unwrap(),
        body.0.active.unwrap(),
        body.0.description.unwrap(),
        body.0.version.unwrap(),
        body.0.source.unwrap()
    ) {
        Ok(res) => HttpResponse::Ok().json(json!({
            "software_id": res,
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))
    }

}