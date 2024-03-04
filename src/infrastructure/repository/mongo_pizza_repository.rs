use mongodb::options::ClientOptions;
use async_trait::async_trait;

use mongodb::{Client, Collection};
use mongodb::bson::{doc, Bson, Document};
use futures::stream::TryStreamExt;

use crate::domain::entity::pizza::Pizza;

use super::pizza_repository::{Repository, RepositoryError};

pub struct MongoRepository {}

impl MongoRepository {
    async fn connect() -> Collection<Document> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("main");
        let collection = db.collection::<Document>("pizzas");
        return collection;
    }
}

#[async_trait]
impl Repository for MongoRepository {
    async fn save(&self, pizza: &Pizza) -> Result<(), RepositoryError> {
        let collection = Self::connect().await;

        let document = doc! {
            "uuid": &pizza.uuid,
            "name": &pizza.name,
            "toppings": &pizza.toppings,
        };
        dbg!(&document);
        let result = collection.insert_one(document, None).await;
        dbg!(&result);
        Ok(())
    }

    async fn get_pizzas(&self) -> Result<Vec<Pizza>, RepositoryError> {
        let collection = Self::connect().await;

        let mut cursor = collection.find(None, None).await.unwrap();

        let mut result = Vec::new();

        while let Some(document) = cursor.try_next().await.unwrap() {

            let uuid = document.get_str("uuid").unwrap().to_string();
            let name = document.get_str("name").unwrap().to_string();
            
            let toppings_array = document.get_array("toppings").unwrap();

            dbg!(&toppings_array);
    
            let mut toppings = Vec::new();
            for bson in toppings_array {
                if let Bson::String(topping) = bson {
                    toppings.push(topping.to_string());
                }
            }
    
            result.push(Pizza { uuid, name, toppings });
        }
    
        Ok(result)
    }   
}