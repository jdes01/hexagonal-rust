use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::web::Json;
use serde::Deserialize;

use crate::application::use_cases::create_pizza::{create_pizza_handler, CreatePizzaCommand};

#[derive(Deserialize)]
pub struct CreatePizzaDTO {
    name: String,
}


pub async fn create_pizza_controller(request_body: Json<CreatePizzaDTO>) -> impl Responder {

    let command: CreatePizzaCommand = CreatePizzaCommand {
        name: String::from(&request_body.name),
    };

    match create_pizza_handler(command).await {
        Ok(uuid) => {
            HttpResponse::Ok().body(format!("Pizza {} creada exitosamente", uuid))
        }
        Err(_) => {
            HttpResponse::NotFound().body("Something went wrong")
        }
    }

    
}