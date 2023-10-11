use std::sync::Mutex;
use actix_web::{HttpResponse, web};
use crate::database_controller::{Database, RequestFilter};

pub async fn all_requests(
    pool: web::Data<Mutex<Database>>,
    path: web::Path<RequestFilter>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let filter = path.into_inner();
    let response = db.get_all_requests(filter);
    HttpResponse::Ok().json(response)
}