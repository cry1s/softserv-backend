use actix_web::{web, App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(
                softserv::init_handlebars()
            ))
            .service(
                fs::Files::new("/static", "./resources/static")
            )
            .service(softserv::index)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}