use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, middleware::Logger, post, Responder, web};
use actix_files as fs;
use handlebars::Handlebars;
use serde::Deserialize;
use database_controller::Database;
use methods::requests::get_all_requests;
use crate::methods::others::not_found;
use crate::methods::softwares::SoftwareFilter;
use crate::models::db_types::Software;
use crate::models::web_types::SoftwareCard;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(
                view::init_handlebars()
            ))
            .app_data(web::Data::new(
                Mutex::new(Database::new())
            ))
            .default_service(
                web::route().to(not_found)
            )
            .service(
                fs::Files::new("/static", "./resources/static")
            )
            .service(index)
            .service(get_soft)
            .service(delete_soft)
            .route("/all_requests", web::get().to(get_all_requests))
            
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    
}

pub mod database_controller;
pub mod models;
pub mod schema;
pub mod view;
pub mod methods;

#[derive(Deserialize)]
struct IndexQuery {
    q: Option<String>,
}

#[get("/")]
async fn index(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<Mutex<Database>>,
    mut query: web::Query<IndexQuery>,
) -> impl Responder {
    let search = query.q.take().unwrap_or("".to_string());
    let software_list = {
        if !search.is_empty() {
            pool.lock().unwrap().get_softwares_by_name(&search)
        } else {
            pool.lock().unwrap().get_all_active_softwares(SoftwareFilter {
                search: None
            })
        }
    };
    let software_list = software_list
        .into_iter()
        .map(|software| SoftwareCard::new(software, pool.clone()))
        .collect();

    view::index(hb, software_list, search)
}
#[get("/soft/{soft_id}")]
pub async fn get_soft(
    hb: web::Data<Handlebars<'_>>,
    pool: web::Data<Mutex<Database>>,
    path: web::Path<(i32,)>,
) -> impl Responder {
    let (id,) = path.into_inner();
    let soft: Option<Software> = pool.lock().unwrap().get_software_by_id(id);
    match soft {
        Some(soft) => view::soft(hb, SoftwareCard::new(soft, pool)),
        None => view::not_found(hb),
    }
}

#[derive(Deserialize)]
pub struct DeleteSoftPayload {
    soft_id: i32,
}

#[post("/delete_soft/")]
pub async fn delete_soft(
    pool: web::Data<Mutex<Database>>,
    payload: web::Json<DeleteSoftPayload>
) -> impl Responder {
    pool.lock().unwrap().delete_software(payload.soft_id);
    HttpResponse::Ok()
}
