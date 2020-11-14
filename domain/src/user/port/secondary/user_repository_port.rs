use crate::user::model::Person;

pub trait UserRepositoryPort {
    fn save(&self, person: Person) -> Person;
}