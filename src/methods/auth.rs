use std::{sync::{Mutex, Arc}, env};

use actix_web::{web, HttpResponse, HttpRequest};
use jsonwebtoken::{DecodingKey, Validation};
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
    req: HttpRequest,
) -> HttpResponse {
    let token = req.headers().get("Authorization").unwrap().to_str().unwrap().trim_start_matches(BEARER);

    let payload = jsonwebtoken::decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
        &Validation::default(),
    );

    if let Ok(data) = payload {
        let tkid = data.claims.tkid;
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
