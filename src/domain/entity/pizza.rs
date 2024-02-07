use serde::{ser::SerializeStruct, Serialize, Serializer};

use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub name: String,
    pub toppings: Vec<String>
}

impl Pizza {
    pub fn new(name: &str) -> Pizza {
        let uuid = Uuid::new_v4().to_string();
        Pizza {
            uuid: String::from(uuid),
            name: String::from(name),
            toppings: [].to_vec()
        }
    }
    pub fn add_toppings(&mut self, topping_names: &Vec<String>) {
        for topping_name in topping_names {
            self.toppings.push(topping_name.to_string())
        }
    }
}

impl Serialize for Pizza {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Pizza", 2)?;
        state.serialize_field("uuid", &self.uuid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("toppings", &serde_json::to_value(&self.toppings).unwrap())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::entity::pizza::Pizza;

    #[test]
    fn test_new() {
        let pizza: Pizza = Pizza::new("peperoni");
        assert_eq!(pizza.name, "peperoni");
        assert_eq!(pizza.name, "peperoni");
    }
}