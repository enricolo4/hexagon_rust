use crate::user::model::Person;

pub trait UserPort {
    fn save(&self, person: Person) -> Person;
}