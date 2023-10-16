use actix_web::{Responder, web};
use handlebars::Handlebars;
use crate::view;

pub async fn not_found(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    view::not_found(hb)
}