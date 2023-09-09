use actix_web::{web, get, Responder, HttpResponse};
use serde::Serialize;
use handlebars::Handlebars;

pub fn init_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("index", "resources/templates/index.hbs").expect("Failed to register template");
    handlebars
}

// Структура для представления данных о софте
#[derive(Serialize)]
struct Software {
    name: String,
    version: String,
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let software_list = vec![
        Software {
            name: "Программа 1".to_string(),
            version: "1.0".to_string(),
        },
        Software {
            name: "Программа 2".to_string(),
            version: "2.0".to_string(),
        },
    ];

    let body = hb.render("index", &software_list);
    
    match body {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
