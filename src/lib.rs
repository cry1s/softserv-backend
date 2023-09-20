use actix_web::{get, web, Responder};
use handlebars::Handlebars;
use serde::Deserialize;

mod model;
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
async fn index(hb: web::Data<Handlebars<'_>>, mut query: web::Query<IndexQuery>) -> impl Responder {
    let search = query.q.take().unwrap_or("".to_string());
    let software_list = model::get_soft_list(&search);
    view::index(hb, software_list, search)
}

#[get("/soft/{soft_id}")]
async fn soft(hb: web::Data<Handlebars<'_>>, path: web::Path<(i32,)>) -> impl Responder {
    let (id,) = dbg!(path.into_inner());
    let soft = model::get_soft_by_id(id);
    let answ = match soft {
        Some(soft) => view::soft(hb, soft),
        None => view::not_found(hb)
    };
    answ
}
