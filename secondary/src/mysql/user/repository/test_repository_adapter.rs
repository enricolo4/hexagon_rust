use domain::user::model::Person;
use crate::mysql::user::dbo::PersonToDBO;
use domain::user::port::secondary::TestRepositoryPort;

pub struct TestRepositoryAdapter;

impl TestRepositoryAdapter {
    pub fn new() -> Self {
        return Self {};
    }
}

impl TestRepositoryPort for TestRepositoryAdapter {
    fn save(&self, person: Person) -> Person {
        let person_dbo = person.to_dbo();
        println!("{:?} Test Controller Salvo", person_dbo);
        person_dbo.to_model()
    }
}