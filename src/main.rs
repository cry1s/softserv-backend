use std::env;
use std::sync::{Mutex, Arc};

use crate::models::Software;
use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use controller::Database;
use dotenvy::dotenv;
use redis::ConnectionInfo;
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
                web::get().to(methods::softwares::all_softwares),
            )
            .route(
                "/software",
                web::post().to(methods::softwares::new_software),
            )
            .service(
                web::resource("/software/{id}")
                    .route(web::get().to(methods::softwares::get_software))
                    .route(web::put().to(methods::softwares::update_software))
                    .route(web::delete().to(methods::softwares::delete_software)),
            )
            .route(
                "/software/{soft_id}/add_tag/{tag_id}",
                web::post().to(methods::softwares::add_tag_to_software),
            )
            .route(
                "/software/{soft_id}/remove_tag/{tag_id}",
                web::delete().to(methods::softwares::delete_tag),
            )
            .route(
                "/software/{soft_id}/add_image",
                web::put().to(methods::softwares::add_image),
            )
            .route(
                "/requests",
                web::get().to(methods::requests::get_all_requests),
            )
            .route("/request", web::post().to(methods::requests::new_request))
            .route(
                "/request/add",
                web::post().to(methods::requests::add_software_to_last_request),
            )
            .service(
                web::resource("/request/{id}")
                    .route(web::get().to(methods::requests::get_request))
                    .route(web::put().to(methods::requests::update_request))
                    .route(web::delete().to(methods::requests::delete_request))
                    .route(web::patch().to(methods::requests::change_request_status)),
            )
            .route(
                "/request/{request_id}/add_software/",
                web::post().to(methods::requests::add_software_to_request),
            )
            .route(
                "/request/{request_id}/remove_software/{software_id}",
                web::delete().to(methods::requests::delete_software_from_request),
            )
            .route(
                "/request/{request_id}/apply_mod",
                web::patch().to(methods::requests::apply_mod),
            )
            .route("/tags/{input}", web::get().to(methods::tags::tags_by_input))
            .route("/tag", web::post().to(methods::tags::new_tag))
            .service(
                web::resource("/tag/{id}")
                    .route(web::get().to(methods::tags::get_tag))
                    .route(web::put().to(methods::tags::update_tag)),
            )
            .route("/auth/register", web::post().to(methods::auth::register))
            .route("/auth/login", web::post().to(methods::auth::login))
            .route("/auth/logout", web::post().to(methods::auth::logout))
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
    let redis_connection = redis::Client::open(redis_host).unwrap().get_tokio_connection().await.unwrap();
    Arc::new(Mutex::new(redis_connection))
}
