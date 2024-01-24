use std::sync::Mutex;

use crate::models::Software;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use controller::Database;
use s3::creds::Credentials;
use s3::{Bucket, Region};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(Mutex::new(Database::new())))
            .app_data(web::Data::new(connect_to_bucket()))
            .default_service(web::route().to(HttpResponse::NotFound))
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
            )
            .route("/request/{id}/user", web::patch().to(methods::requests::change_request_status))
            .route("/request/{id}/admin", web::patch().to(methods::requests::change_request_status_admin))
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
            access_key: Some("sglwG7iKSo4jJv9aedym".to_string()),
            secret_key: Some("L7IL1UjCaSUaiZCZjlN27vdgEOfSLp7nSCgZqdj9".to_string()),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
    .unwrap()
    .with_path_style()
}
