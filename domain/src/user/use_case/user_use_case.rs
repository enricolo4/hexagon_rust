use crate::user::model::Person;
use crate::user::port::primary::UserPort;
use crate::user::port::secondary::UserRepositoryPort;
use std::sync::Arc;

pub struct UserUseCase {
    adapter: Arc<dyn UserRepositoryPort>
}

impl UserUseCase {
    pub fn new(adapter: Arc<dyn UserRepositoryPort>) -> Self {
        return Self { adapter };
    }
}

impl UserPort for UserUseCase {
    fn save(&self, person: Person) -> Person {
        self.adapter.save(person)
    }
}
