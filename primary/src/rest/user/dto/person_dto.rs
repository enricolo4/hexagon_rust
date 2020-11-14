use domain::user::model::Person;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonDTO {
    pub name: String,
    pub age: u32,
    pub cpf: String,
    pub email: String,
}

impl PersonDTO {
    pub fn new(name: String, age: u32, cpf: String, email: String) -> Self {
        Self { name, age, cpf, email }
    }

    pub fn to_model(&self) -> Person {
        Person::new(
            self.name.to_string(),
            self.age,
            self.cpf.to_string(),
            self.email.to_string(),
        )
    }
}

pub trait PersonToDTO {
    fn to_dto(&self) -> PersonDTO;
}

impl PersonToDTO for Person {
    fn to_dto(&self) -> PersonDTO {
        PersonDTO::new(
            self.name.to_string(),
            self.age,
            self.cpf.to_string(),
            self.email.to_string(),
        )
    }
}