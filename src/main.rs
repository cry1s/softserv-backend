use actix_web::{web, App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(
                softserv::init_handlebars()
            ))
            .default_service(
                web::route().to(softserv::not_found)
            )
            .service(
                fs::Files::new("/static", "./resources/static")
            )
            .service(softserv::index)
            .service(softserv::soft)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}