use std::sync::Arc;

use crate::domain::entity::pizza::Pizza;
use crate::infrastructure::repository::pizza_repository::Repository;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreatePizzaCommand {
    pub name: String,
    pub toppings: Vec<String>
}

#[derive(Clone)]
pub struct CreatePizzaHandler<'a> {
    repository: Arc<dyn Repository + Sync + Send>,
    _phantom: std::marker::PhantomData<&'a ()>, 
}

impl<'a> CreatePizzaHandler<'a> {
    pub fn new(repository: Arc<dyn Repository + Sync + Send>) -> Self {
        CreatePizzaHandler {
            repository,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn execute(&self, command: CreatePizzaCommand) -> Result<String, ()> {
        let mut pizza = Pizza::new(&command.name);
        pizza.add_toppings(&command.toppings);

        let cloned_pizza = pizza.clone();

        let repository = Arc::clone(&self.repository);
        tokio::spawn(async move {
            repository.save(&cloned_pizza).await.unwrap();
        });

        Ok(pizza.uuid)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::time::Duration;
    use crate::{
        application::use_cases::create_pizza_handler::{CreatePizzaHandler, CreatePizzaCommand},
        infrastructure::repository::pizza_repository::{InMemoryRepository, Repository},
    };

    #[tokio::test]
    async fn test_create_pizza_success() {
        let repository = Arc::new(InMemoryRepository::default());

        let command = CreatePizzaCommand {
            name: "Margherita".to_string(),
            toppings: vec!["Tomato".to_string(), "Mozzarella".to_string()],
        };

        let handler = CreatePizzaHandler::new(repository.clone());
        let result = handler.execute(command).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_pizza_repository() {
        let repository = Arc::new(InMemoryRepository::default());

        let command = CreatePizzaCommand {
            name: "Margherita".to_string(),
            toppings: vec!["Tomato".to_string(), "Mozzarella".to_string()],
        };

        let handler = CreatePizzaHandler::new(repository.clone());
        handler.execute(command).await.unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;

        let pizzas = repository.get_pizzas().await.unwrap();
        assert_eq!(pizzas.len(), 1);

        let saved_pizza = pizzas.first().unwrap();
        assert_eq!(saved_pizza.name, "Margherita");
        assert_eq!(saved_pizza.toppings, vec!["Tomato", "Mozzarella"]);
    }
}