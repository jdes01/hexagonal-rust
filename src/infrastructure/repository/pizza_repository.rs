use std::fmt;
use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;
use async_trait::async_trait;

use crate::domain::entity::pizza::Pizza;

#[derive(Debug, Clone)]
pub struct RepositoryError;

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong")
    }
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn save(&self, pizza: &Pizza) -> Result<(), RepositoryError>;
    async fn get_pizzas(&self) -> Result<Vec<Pizza>, RepositoryError>;
}

#[derive(Clone, Default)]
pub struct InMemoryRepository {
    pizzas: Arc<Mutex<Vec<Pizza>>>,
}

#[async_trait::async_trait]
impl Repository for InMemoryRepository {
    async fn save(&self, pizza: &Pizza) -> Result<(), RepositoryError> {
        let mut pizzas = self.pizzas.lock().unwrap();
        pizzas.push(pizza.clone());
        Ok(())
    }

    async fn get_pizzas(&self) -> Result<Vec<Pizza>, RepositoryError> {
        let pizzas = self.pizzas.lock().unwrap();
        Ok(pizzas.clone())
    }
}


#[derive(Clone)]
pub struct PostgresRepository {}

impl PostgresRepository {
    async fn connect() -> tokio_postgres::Client {
        let db_url = "postgres://username:password@localhost/my_database";
        let (client, connection) = tokio_postgres::connect(db_url, NoTls)
            .await
            .expect("Error al conectar a la base de datos");
    
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error de conexión a la base de datos: {}", e);
            }
        });
        client
    }

    async fn create_tables() {
        let client = Self::connect().await;
        client
            .execute(
                r#"CREATE TABLE IF NOT EXISTS pizzas (
                    "id" VARCHAR(255) PRIMARY KEY,
                    "name" VARCHAR(255) NOT NULL,
                    "toppings" VARCHAR(255)[]
                )"#,
                &[],
            )
            .await
            .expect("Error al crear la tabla pizzas");
    }
}

#[async_trait]
impl Repository for PostgresRepository {

    async fn save(&self, pizza: &Pizza) -> Result<(), RepositoryError> {
        Self::create_tables().await;
        let client = Self::connect().await;
        client
            .execute(
                "INSERT INTO pizzas (id, name, toppings) VALUES ($1, $2, $3)",
                &[&pizza.uuid, &pizza.name, &&pizza.toppings],
            )
            .await
            .expect("Error al insertar la pizza");
        Ok(())
    }

    async fn get_pizzas(&self) -> Result<Vec<Pizza>, RepositoryError> {
        let client = Self::connect().await;
        let stmt = client
            .prepare("SELECT id, name, toppings FROM pizzas")
            .await
            .expect("Error al preparar la consulta SELECT");
        let rows = client
            .query(&stmt, &[])
            .await
            .expect("Error al ejecutar la consulta SELECT");
    
        let pizzas: Vec<Pizza> = rows
            .iter()
            .map(|row| {

                let uuid: String = row.get(0);
                let name: String = row.get(1);
                let toppings: Vec<String> = row.get(2);
    
                Pizza { uuid, name, toppings }
            })
            .collect();
    
        Ok(pizzas)
    }
}
