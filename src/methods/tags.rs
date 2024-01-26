use std::sync::Mutex;

use actix_web::web::ReqData;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::controller::Database;
use crate::methods::Response;
use crate::models::TokenClaims;

pub(crate) async fn all_tags(
    pool: web::Data<Mutex<Database>>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let tags = db.get_all_tags();
    HttpResponse::Ok().json(tags)
}

#[derive(Deserialize)]
pub(crate) struct Tag {
    pub(crate) name: String,
}

pub(crate) async fn new_tag(
    pool: web::Data<Mutex<Database>>,
    body: web::Json<Tag>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    if !claims.unwrap().moderator {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно прав"
        }));
    }
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
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    if !claims.unwrap().moderator {
        return HttpResponse::BadRequest().json(json!({
            "error": "Недостаточно прав"
        }));
    }
    let mut db = pool.lock().unwrap();
    let tag = db.update_tag_by_id(path.into_inner(), tag.name.clone());
    match tag {
        Ok(tag) => HttpResponse::Ok().json(tag),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}
