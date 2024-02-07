use std::sync::Arc;
use crate::infrastructure::repository::pizza_repository::{PostgresRepository, Repository};
use crate::application::use_cases::{
    get_pizzas_handler::GetPizzasHandler,
    create_pizza_handler::CreatePizzaHandler
};
use lazy_static::lazy_static;


pub enum RepositoryType {
    Postgres,
}


#[derive(Clone)]
pub struct Container<'a> {
    create_pizza_handler: CreatePizzaHandler<'a>,
    get_pizzas_handler: GetPizzasHandler,
}

impl<'a> Container<'a> {
    pub fn new(repository_type: RepositoryType) -> Self {
        let repository: Arc<dyn Repository + Send + Sync> = match repository_type {
            RepositoryType::Postgres => Arc::new(PostgresRepository {}),
        };

        let create_pizza_handler = CreatePizzaHandler::new(repository.clone());
        let get_pizzas_handler = GetPizzasHandler::new(repository);

        Container {
            create_pizza_handler,
            get_pizzas_handler,
        }
    }

    pub fn create_pizza_handler(&self) -> &CreatePizzaHandler<'a> {
        &self.create_pizza_handler
    }

    pub fn get_pizzas_handler(&self) -> &GetPizzasHandler {
        &self.get_pizzas_handler
    }
}

lazy_static! {
    pub static ref CONTAINER: Container<'static> = {
        let container = Container::new(RepositoryType::Postgres);
        container
    };
}