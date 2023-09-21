use std::env;

use actix_web::{get, web, Responder};
use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use handlebars::Handlebars;
use serde::Deserialize;

use crate::models::Software;

pub mod models;
pub mod schema;
mod view;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

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
async fn index(hb: web::Data<Handlebars<'_>>, mut query: web::Query<IndexQuery>) -> impl Responder {
    let search = query.q.take().unwrap_or("".to_string());
    let software_list: Vec<Software> = vec![];
    view::index(hb, software_list, search)
}

#[get("/soft/{soft_id}")]
async fn soft(hb: web::Data<Handlebars<'_>>, path: web::Path<(i32,)>) -> impl Responder {
    let (_id,) = path.into_inner();
    let soft: Option<Software> = Some(Software::default());
    let answ = match soft {
        Some(soft) => view::soft(hb, soft),
        None => view::not_found(hb)
    };
    answ
}
