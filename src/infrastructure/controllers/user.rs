use actix_web::{web, Responder};

async fn user_hello() -> impl Responder {
    "Hello world! user"
}

pub fn user_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(user_hello)),
    );
}