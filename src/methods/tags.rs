use std::sync::Mutex;

use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::controller::Database;
use crate::methods::Response;

pub(crate) async fn tags_by_input(
    pool: web::Data<Mutex<Database>>,
    input: web::Path<String>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let tags = db.get_tags_by_input(input.into_inner());
    HttpResponse::Ok().json(tags)
}

#[derive(Deserialize)]
pub(crate) struct Tag {
    pub(crate) name: String,
}

pub(crate) async fn new_tag(
    pool: web::Data<Mutex<Database>>,
    body: web::Json<Tag>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let tag = db.create_tag(body.name.clone());
    tag.response(json!({
        "status": "ok",
    }))
}

pub(crate) async fn get_tag(
    pool: web::Data<Mutex<Database>>,
    path: web::Path<i32>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let tag = db.get_tag_by_id(path.into_inner());
    match tag {
        Some(tag) => HttpResponse::Ok().json(tag),
        None => HttpResponse::BadRequest().json("Tag not found"),
    }
}

pub(crate) async fn update_tag(
    pool: web::Data<Mutex<Database>>,
    path: web::Path<i32>,
    tag: web::Json<Tag>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let tag = db.update_tag_by_id(path.into_inner(), tag.name.clone());
    match tag {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}
