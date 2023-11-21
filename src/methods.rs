use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

pub mod requests;
pub mod softwares;
pub mod tags;

trait Response {
    fn response(&self, ok: impl Serialize) -> HttpResponse;
}

impl<T, E: std::fmt::Display> Response for Result<T, E> {
    fn response(&self, ok: impl Serialize) -> HttpResponse {
        match self {
            Ok(_) => HttpResponse::Ok().json(ok),
            Err(e) => HttpResponse::InternalServerError().json(json!({
                "error": e.to_string()
            })),
        }
    }
}
