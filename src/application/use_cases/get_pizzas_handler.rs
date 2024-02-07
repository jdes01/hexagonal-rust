use std::sync::Arc;
use crate::{
    domain::entity::pizza::Pizza,
    infrastructure::repository::pizza_repository::Repository,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetPizzasQuery {}

#[derive(Clone)]
pub struct GetPizzasHandler {
    repository: Arc<dyn Repository>,
}

impl GetPizzasHandler {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        GetPizzasHandler { repository }
    }

    pub async fn execute(&self, _: GetPizzasQuery) -> Option<Vec<Pizza>> {
        match self.repository.get_pizzas().await {
            Ok(pizzas) => Some(pizzas),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::time::Duration;
    use crate::{
        application::use_cases::get_pizzas_handler::{GetPizzasHandler, GetPizzasQuery},
        domain::entity::pizza::Pizza,
        infrastructure::repository::pizza_repository::{InMemoryRepository, Repository},
    };

    #[tokio::test]
    async fn test_get_pizzas_handler_success() {
        let pizza = Pizza { name: "pizza".to_string(), uuid: "uuid".to_string(), toppings: Vec::new() };
        let repository = Arc::new(InMemoryRepository::default());
        repository.save(&pizza).await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        let handler = GetPizzasHandler::new(repository.clone());
        let query = GetPizzasQuery {};
        let result = handler.execute(query).await;

        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_pizzas_handler_empty_repo() {
        let empty_repo = Arc::new(InMemoryRepository::default());
        let handler = GetPizzasHandler::new(empty_repo.clone());
        let query = GetPizzasQuery {};
        let result = handler.execute(query).await;

        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 0);
    }
}
