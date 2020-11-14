use crate::user::model::Person;
use crate::user::port::primary::UserPort;
use crate::user::port::secondary::UserRepositoryPort;

pub struct UserUseCase {
    adapter: Box<dyn UserRepositoryPort>
}

impl UserUseCase {
    pub fn new(adapter: Box<dyn UserRepositoryPort>) -> Self {
        return Self { adapter };
    }
}

impl UserPort for UserUseCase {
    fn save(&self, person: Person) -> Person {
        self.adapter.save(person)
    }
}
