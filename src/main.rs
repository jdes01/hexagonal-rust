use actix_web::{web, App, HttpResponse, HttpServer};

mod infrastructure;
mod application;
mod domain;
mod container;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(container.clone()))
            .configure(infrastructure::routes::pizza::pizza_routes)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })

    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}