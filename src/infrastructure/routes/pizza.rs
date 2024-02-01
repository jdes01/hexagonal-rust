use actix_web::web;

use crate::infrastructure::controllers::pizza::create_pizza_controller;

pub fn pizza_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/pizza")
            .route(web::post().to(create_pizza_controller)),
    );
}