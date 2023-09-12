use actix_web::{web, get, Responder, HttpResponse};
use handlebars::Handlebars;
use serde::Serialize;

use crate::model::Software;

mod model;

pub fn init_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "resources/templates/index.hbs").expect("Failed to register template");
    handlebars
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let software_list = model::get_software_list();

    #[derive(Serialize)]
    struct RenderData {
        software_list: Vec<Software>
    }

    let body = hb.render("index", &RenderData {
        software_list
    });
    
    match body {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
