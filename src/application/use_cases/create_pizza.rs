use std::fmt::Error;

use uuid::Uuid;
use crate::domain::entity::pizza::Pizza;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePizzaCommand {
    pub name: String,
}

pub async fn create_pizza_handler(command: CreatePizzaCommand) -> Result<Uuid, Error> {
    let pizza: Pizza = Pizza::new(&command.name);

    Ok(pizza.uuid)
}