use std::sync::Mutex;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files as fs;
use softserv::{database_controller::Database, index, get_soft, delete_soft};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(
                softserv::init_handlebars()
            ))
            .app_data(web::Data::new(
                Mutex::new(Database::new())
            ))
            .default_service(
                web::route().to(softserv::not_found)
            )
            .service(
                fs::Files::new("/static", "./resources/static")
            )
            .service(index)
            .service(get_soft)
            .service(delete_soft)
            
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    
}