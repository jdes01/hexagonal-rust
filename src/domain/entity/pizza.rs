use uuid::Uuid;

pub struct Pizza {
    pub uuid: Uuid,
    pub name: String,
}

impl Pizza {
    pub fn new(name: &str) -> Pizza {
        Pizza {
            uuid: Uuid::new_v4(),
            name: String::from(name),
        }
    }
}