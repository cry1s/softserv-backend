use std::env;
use std::sync::{Arc, Mutex};

use crate::models::Software;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use controller::Database;
use dotenvy::dotenv;
use methods::auth::middleware::VerifyAuth;
use s3::creds::Credentials;
use s3::{Bucket, Region};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let redis_connection = connect_to_redis().await;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(Mutex::new(Database::new())))
            .app_data(web::Data::new(connect_to_bucket()))
            .app_data(web::Data::new(redis_connection.clone()))
            .default_service(web::route().to(HttpResponse::NotFound))
            .service(Files::new("/swagger-ui", "./swagger-ui").index_file("index.html"))
            .route(
                "/softwares",
                web::get()
                    .to(methods::softwares::all_softwares)
                    .wrap(VerifyAuth::optional()),
            )
            .route(
                "/software",
                web::post()
                    .to(methods::softwares::new_software)
                    .wrap(VerifyAuth::required()),
            )
            .service(
                web::resource("/software/{id}")
                    .route(
                        web::get()
                            .to(methods::softwares::get_software)
                            .wrap(VerifyAuth::optional()),
                    )
                    .route(
                        web::put()
                            .to(methods::softwares::update_software)
                            .wrap(VerifyAuth::required()),
                    )
                    .route(
                        web::delete()
                            .to(methods::softwares::delete_software)
                            .wrap(VerifyAuth::required()),
                    ),
            )
            .route(
                "/software/{soft_id}/add_tag/{tag_id}",
                web::post()
                    .to(methods::softwares::add_tag_to_software)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/software/{soft_id}/remove_tag/{tag_id}",
                web::delete()
                    .to(methods::softwares::delete_tag)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/software/{soft_id}/add_image",
                web::put()
                    .to(methods::softwares::add_image)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/requests",
                web::get()
                    .to(methods::requests::get_all_requests)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/request",
                web::post()
                    .to(methods::requests::new_request)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/request/add",
                web::post()
                    .to(methods::requests::add_software_to_last_request)
                    .wrap(VerifyAuth::required()),
            )
            .service(
                web::resource("/request/{id}")
                    .route(web::get().to(methods::requests::get_request))
                    .route(web::put().to(methods::requests::update_request))
                    .route(web::delete().to(methods::requests::delete_request))
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/request/{id}/user",
                web::patch()
                    .to(methods::requests::change_request_status)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/request/{id}/admin",
                web::patch()
                    .to(methods::requests::change_request_status_admin)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/request/{request_id}/add_software/",
                web::post()
                    .to(methods::requests::add_software_to_request)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/request/{request_id}/remove_software/{software_id}",
                web::delete()
                    .to(methods::requests::delete_software_from_request)
                    .wrap(VerifyAuth::required()),
            )
            .route(
                "/request/{request_id}/apply_mod",
                web::patch()
                    .to(methods::requests::apply_mod)
                    .wrap(VerifyAuth::required()),
            )
            .route("/tags", web::get().to(methods::tags::all_tags))
            .route(
                "/tag",
                web::post()
                    .to(methods::tags::new_tag)
                    .wrap(VerifyAuth::required()),
            )
            .service(
                web::resource("/tag/{id}")
                    .route(
                        web::get()
                            .to(methods::tags::get_tag)
                            .wrap(VerifyAuth::optional()),
                    )
                    .route(
                        web::put()
                            .to(methods::tags::update_tag)
                            .wrap(VerifyAuth::required()),
                    ),
            )
            .route(
                "/auth/register",
                web::post()
                    .to(methods::auth::register)
                    .wrap(VerifyAuth::optional()),
            )
            .route(
                "/auth/login",
                web::post()
                    .to(methods::auth::login)
                    .wrap(VerifyAuth::optional()),
            )
            .route(
                "/auth/logout",
                web::post()
                    .to(methods::auth::logout)
                    .wrap(VerifyAuth::required()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}   

pub(crate) mod controller;
pub(crate) mod methods;
pub(crate) mod models;
pub(crate) mod schema;

fn connect_to_bucket() -> Bucket {
    Bucket::new(
        "bucket",
        Region::Custom {
            region: "minio".to_string(),
            endpoint: "http://localhost:9000".to_owned(),
        },
        Credentials {
            access_key: env::var("MINIO_ROOT_USER").ok(),
            secret_key: env::var("MINIO_ROOT_PASSWORD").ok(),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
    .unwrap()
    .with_path_style()
}

async fn connect_to_redis() -> Arc<Mutex<redis::aio::Connection>> {
    let redis_host = format!(
        "redis://:{}@{}:{}",
        env::var("REDIS_PASSWORD").unwrap(),
        env::var("REDIS_HOST").unwrap(),
        env::var("REDIS_PORT").unwrap()
    );
    println!("Connecting to redis at {}", redis_host);
    let redis_connection = redis::Client::open(redis_host)
        .unwrap()
        .get_tokio_connection()
        .await
        .unwrap();
    Arc::new(Mutex::new(redis_connection))
}
