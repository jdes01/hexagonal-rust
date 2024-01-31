use actix_web::web;

use crate::application::use_cases::get_pizzas::pizza_hello;

pub fn pizza_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/pizza")
            .route(web::get().to(pizza_hello)),
    );
}