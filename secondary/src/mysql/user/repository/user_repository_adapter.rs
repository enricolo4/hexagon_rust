use domain::user::model::Person;
use domain::user::port::secondary::UserRepositoryPort;
use crate::mysql::user::dbo::PersonToDBO;

#[derive(Clone, Copy)]
pub struct UserRepositoryAdapter;

impl UserRepositoryAdapter {
    pub fn new() -> Self {
        return Self {};
    }
}

impl UserRepositoryPort for UserRepositoryAdapter {
    fn save(&self, person: Person) -> Person {
        let person_dbo = person.to_dbo();
        println!("{:?} Salvo", person_dbo);
        person_dbo.to_model()
    }
}