use domain::user::port::primary::TestPort;
use actix_web::Responder;
use actix_web::web::{Json, Data};
use std::sync::Arc;
use crate::rest::user::dto::{PersonDTO, PersonToDTO};

pub struct TestController {
    user_port: Arc<dyn TestPort>
}

impl TestController {
    pub fn new(user_port: Arc<dyn TestPort>) -> Self { Self { user_port } }

    pub async fn save(user_controller: Data<Arc<TestController>>, person_dto: Json<PersonDTO>) -> impl Responder {
        let response = user_controller.save_new_person(person_dto);
        Json(response)
    }

    fn save_new_person(&self, person_dto: Json<PersonDTO>) -> PersonDTO {
        self.user_port.save(person_dto.0.to_model()).to_dto()
    }
}
