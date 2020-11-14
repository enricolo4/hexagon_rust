use domain::user::model::Person;
use uuid::Uuid;

#[derive(Debug)]
pub struct PersonDBO {
    pub id: Uuid,
    pub name: String,
    pub age: u32,
    pub cpf: String,
    pub email: String,
}

impl PersonDBO {
    pub fn new(id: Uuid, name: String, age: u32, cpf: String, email: String) -> Self {
        Self { id, name, age, cpf, email }
    }

    pub fn to_model(&self) -> Person {
        Person::new(
            self.name.to_string(),
            self.age,
            self.cpf.to_string(),
            self.email.to_string())
    }
}

pub trait PersonToDBO {
    fn to_dbo(&self) -> PersonDBO;
}

impl PersonToDBO for Person {
    fn to_dbo(&self) -> PersonDBO {
        PersonDBO::new(
            Uuid::new_v4(),
            self.name.to_string(),
            self.age,
            self.cpf.to_string(),
            self.email.to_string(),
        )
    }
}



