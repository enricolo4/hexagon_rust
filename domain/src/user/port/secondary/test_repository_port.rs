use crate::user::model::Person;
use shaku::Interface;

pub trait TestRepositoryPort: Interface {
    fn save(&self, person: Person) -> Person;
}