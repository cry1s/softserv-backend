use actix_web::{web, get, Responder, HttpResponse};
use handlebars::Handlebars;

mod model;
mod view;

pub fn init_handlebars() -> Handlebars<'static> {
    let msg = "Failed to register template";
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "resources/templates/index.hbs").expect(msg);
    handlebars.register_template_file("layout", "resources/templates/layout.hbs").expect(msg);
    handlebars
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    match view::index(hb) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
