use actix_web::{Responder, HttpResponse, web::Json};
use serde::Deserialize;
use serde_json;
use crate::{
    application::use_cases::get_pizzas_handler::GetPizzasQuery, container::container::CONTAINER
};

#[derive(Deserialize)]
pub struct GetPizzasDTO {}

pub async fn get_pizzas_controller(_: Json<GetPizzasDTO>) -> impl Responder {

    let query: GetPizzasQuery = GetPizzasQuery {};

    match CONTAINER.get_pizzas_handler().execute(query).await {
        Some(pizzas) => HttpResponse::Ok().body(serde_json::to_string(&pizzas).unwrap()),
        None => HttpResponse::NotFound().body("No pizzas!"),
    }
}