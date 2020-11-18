use crate::user::model::Person;
use shaku::Interface;

pub trait UserPort: Interface {
    fn save(&self, person: Person) -> Person;
}