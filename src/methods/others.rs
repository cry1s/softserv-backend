use crate::view;
use actix_web::{web, Responder};
use handlebars::Handlebars;

pub(crate) async fn not_found(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    view::not_found(hb)
}
