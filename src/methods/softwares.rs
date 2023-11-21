use crate::controller::Database;
use crate::methods::Response;
use crate::models::OptionInsertSoftware;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt as _};
use serde::Deserialize;
use serde_json::json;
use std::sync::Mutex;

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
    body: web::Json<OptionInsertSoftware>,
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
    let id = id.unwrap();

    let mut db = pool.lock().unwrap();
    let software = db.get_software_by_id(id);
    if software.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error": "ID не существует"
        }));
    }
    let software = software.unwrap();
    let new_data = OptionInsertSoftware {
        name: body.0.name.or(Option::from(software.software.name)),
        active: body.0.active.or(Option::from(software.software.active)),
        description: body
            .0
            .description
            .or(Option::from(software.software.description)),
        version: body.0.version.or(Option::from(software.software.version)),
        source: body.0.source.or(Option::from(software.software.source)),
    };
    let response = db.update_software_by_id(id, new_data);
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn new_software(
    pool: web::Data<Mutex<Database>>,
    body: web::Json<OptionInsertSoftware>,
) -> HttpResponse {
    if body.any_none() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно полей"
        }));
    }

    let mut db = pool.lock().unwrap();
    let res = db.new_software(
        body.0.name.unwrap(),
        body.0.active.unwrap(),
        body.0.description.unwrap(),
        body.0.version.unwrap(),
        body.0.source.unwrap(),
    );
    res.response(json!({
        "status": "ok"
    }))
}

#[derive(Deserialize)]
pub(crate) struct AddTagPayload {
    pub(crate) soft_id: i32,
    pub(crate) tag_id: i32,
}

pub(crate) async fn add_tag_to_software(
    pool: web::Data<Mutex<Database>>,
    path: web::Path<AddTagPayload>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let response = db.add_tag_to_software(path.soft_id, path.tag_id);
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn delete_tag(
    pool: web::Data<Mutex<Database>>,
    path: web::Path<AddTagPayload>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let response = db.delete_tag_from_software(path.soft_id, path.tag_id);
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn delete_software(
    pool: web::Data<Mutex<Database>>,
    path: web::Path<(i32,)>,
) -> impl Responder {
    let response = pool.lock().unwrap().delete_software(path.0);
    response.response(json!({
        "status": "ok"
    }))
}

pub(crate) async fn add_image(
    s3: web::Data<s3::bucket::Bucket>,
    pool: web::Data<Mutex<Database>>,
    mut path: web::Path<(Option<String>,)>,
    mut body: Multipart,
) -> HttpResponse {
    if path.0.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "error:": "Не представлен ID"
        }));
    }
    let id = path.0.take().unwrap().parse::<i32>();
    if id.is_err() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Неправильный ID"
        }));
    }
    let id = id.unwrap();
    let mut file_data = Vec::<u8>::new();
    while let Some(mut field) = body.try_next().await.unwrap() {
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name();
        if field_name.is_none() {
            return HttpResponse::BadRequest().json(json!({
                "error": "Неправильное поле"
            }));
        }
        let field_name = field_name.unwrap();
        if field_name == "file" {
            while let Some(chunk) = field.next().await {
                if chunk.is_err() {
                    return HttpResponse::BadRequest().json(json!({
                        "error": chunk.err().unwrap().to_string()
                    }));
                }
                file_data.extend_from_slice(chunk.unwrap().as_ref());
            }
        }
    }
    let resp = s3.put_object(format!("{}.png", id), &file_data).await;
    if resp.is_err() {
        return HttpResponse::BadRequest().json(json!({
            "error": resp.err().unwrap().to_string()
        }));
    }
    let url = format!("http://localhost:9000/bucket/{}.png", id);
    let mut db = pool.lock().unwrap();
    let response = db.add_logo_to_software(id, &url);
    response.response(json!({
        "status": "ok",
        "url": url
    }))
}
