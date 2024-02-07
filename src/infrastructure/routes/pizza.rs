use actix_web::web;

use crate::infrastructure::controllers::{create_pizza_controller::create_pizza_controller, get_pizzas_controller::get_pizzas_controller};

pub fn pizza_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/pizza")
            .route(web::post().to(create_pizza_controller))
            .route(web::get().to(get_pizzas_controller))
    );
}