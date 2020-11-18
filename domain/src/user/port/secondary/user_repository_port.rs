use crate::user::model::Person;
use shaku::Interface;

pub trait UserRepositoryPort: Interface {
    fn save(&self, person: Person) -> Person;
}