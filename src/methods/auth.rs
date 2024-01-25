use std::sync::{Mutex, Arc};

use actix_web::{web::{self, ReqData}, HttpResponse};
use redis::{aio::Connection, AsyncCommands};
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::{controller::Database, models::TokenClaims};

const BEARER: &str = "Bearer ";

#[derive(Serialize, Deserialize)]
pub(crate) struct LoginRegister {
    pub(crate) username: String,
    pub(crate) password: String,
}

pub(crate) async fn register(
    pool: web::Data<Mutex<Database>>,
    data: web::Json<LoginRegister>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let user = data.into_inner();

    let result = db.register(user);

    match result {
        Ok(_) => HttpResponse::Created().json(json!({
            "message": "User registered successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })),
    }
}

pub(crate) async fn login(
    pool: web::Data<Mutex<Database>>,
    data: web::Json<LoginRegister>,
) -> HttpResponse {
    let mut db = pool.lock().unwrap();
    let user = data.into_inner();

    let result = db.login(user);

    match result {
        Ok(token) => HttpResponse::Ok().json(json!({
            "token": token
        })),
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        })),
    }
}

pub(crate) async fn logout(
    redis_connection: web::Data<Arc<Mutex<Connection>>>,
    claims: Option<ReqData<TokenClaims>>
) -> HttpResponse {
    if let Some(data) = claims {
        let tkid = data.into_inner().tkid;
        let mut redis_connection = redis_connection.lock().unwrap();
        let _ : () = redis_connection.set(&tkid, "blocked").await.unwrap();
        return HttpResponse::Ok().json(json!({
            "message": "User logged out successfully"
        }));
    }
    HttpResponse::InternalServerError().json(json!({
        "error": "Invalid token"
    }))
}

pub(crate) mod middleware;
