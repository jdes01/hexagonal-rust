use actix_web::{web, App, HttpResponse, HttpServer};

mod infrastructure;
mod application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(infrastructure::controllers::pizza::pizza_endpoints)
            .configure(infrastructure::controllers::user::user_endpoints)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
