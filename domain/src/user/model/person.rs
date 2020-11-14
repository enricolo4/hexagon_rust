#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub cpf: String,
    pub email: String,
}

impl Person {
    pub fn new(name: String, age: u32, cpf: String, email: String) -> Self {
        Self { name, age, cpf, email }
    }
}
