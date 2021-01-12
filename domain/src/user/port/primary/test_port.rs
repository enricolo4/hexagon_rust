use crate::user::model::Person;
use shaku::Interface;

pub trait TestPort: Interface {
    fn save(&self, person: Person) -> Person;
}