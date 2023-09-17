use actix_web::{web, HttpResponse};
use handlebars::{Handlebars, RenderError};
use serde_json::json;

use crate::model::Software;

trait Responsive {
    fn response(self) -> HttpResponse; 
}

impl Responsive for Result<String, RenderError> {
    fn response(self) -> HttpResponse {
        match self {
            Ok(body) => HttpResponse::Ok().body(body),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub(crate) fn index(hb: web::Data<Handlebars>, software_list: Vec<Software>) -> HttpResponse {
    hb.render("index", &json!({
        "parent": "layout",
        "software_list": software_list
    })).response()
}

pub(crate) fn soft(hb: web::Data<Handlebars>, soft: Software) -> HttpResponse {
    hb.render("soft", &json!({
        "parent": "layout",
        "soft": dbg!(soft)
    })).response()
}

pub(crate) fn not_found(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    match hb.render("404", &json!({
        "parent": "layout"
    })) {
        Ok(body) => HttpResponse::NotFound().body(body),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}