use std::sync::Arc;
use crate::user::port::primary::TestPort;
use crate::user::model::Person;
use crate::user::port::secondary::TestRepositoryPort;

pub struct TestUseCase {
    adapter: Arc<dyn TestRepositoryPort>
}

impl TestUseCase {
    pub fn new(adapter: Arc<dyn TestRepositoryPort>) -> Self {
        return Self { adapter };
    }
}

impl TestPort for TestUseCase {
    fn save(&self, person: Person) -> Person {
        self.adapter.save(person)
    }
}
