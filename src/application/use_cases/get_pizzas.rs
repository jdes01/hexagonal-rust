use actix_web::Responder;

pub async fn pizza_hello() -> impl Responder {
    "all pizzas"
}