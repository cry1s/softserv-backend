use std::sync::Mutex;

use actix_web::{get, web, Responder};
use handlebars::Handlebars;
use serde::Deserialize;

use crate::{models::Software, database_controller::Database};

pub mod models;
pub mod schema;
pub mod database_controller;
mod view;

pub fn init_handlebars() -> Handlebars<'static> {
    let msg = "Failed to register template";
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("index", "resources/templates/index.hbs")
        .expect(msg);
    handlebars
        .register_template_file("layout", "resources/templates/layout.hbs")
        .expect(msg);
    handlebars
        .register_template_file("404", "resources/templates/404.hbs")
        .expect(msg);
    handlebars
        .register_template_file("soft", "resources/templates/soft.hbs")
        .expect(msg);
    handlebars
}

#[derive(Deserialize)]
struct IndexQuery {
    q: Option<String>,
}

pub async fn not_found(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    view::not_found(hb)
}

#[get("/")]
async fn index(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<Mutex<Database>>,
    mut query: web::Query<IndexQuery>,
) -> impl Responder {
    let search = query.q.take().unwrap_or("".to_string());

    let software_list = pool.lock().unwrap().get_all_softwares();

    view::index(hb, software_list, search)
}

#[get("/soft/{soft_id}")]
async fn soft(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<Mutex<Database>>,
    path: web::Path<(i32,)>
) -> impl Responder {
    let (id,) = path.into_inner();
    let soft: Option<Software> = pool.lock().unwrap().get_software_by_id(id);
    let answ = match soft {
        Some(soft) => view::soft(hb, soft),
        None => view::not_found(hb)
    };
    answ
}
