use actix_web::web;
use handlebars::{Handlebars, RenderError};
use serde_json::json;

pub fn index(hb: web::Data<Handlebars>) -> Result<String, RenderError> {
    let software_list = crate::model::get_software_list();

    hb.render("index", &json!({
        "parent": "layout",
        "software_list": software_list
    }))
}