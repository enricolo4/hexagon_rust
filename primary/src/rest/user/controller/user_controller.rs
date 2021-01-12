use crate::rest::user::dto::{PersonDTO, PersonToDTO};

use domain::user::port::primary::UserPort;
use actix_web::Responder;
use actix_web::web::{Json, Data};
use std::sync::Arc;

pub struct UserController {
    user_port: Arc<dyn UserPort>
}

impl UserController {
    pub fn new(user_port: Arc<dyn UserPort>) -> Self { Self { user_port } }

    pub async fn save(user_controller: Data<Arc<UserController>>, person_dto: Json<PersonDTO>) -> impl Responder {
        let response = user_controller.save_new_person(person_dto);
        Json(response)
    }

    fn save_new_person(&self, person_dto: Json<PersonDTO>) -> PersonDTO {
        self.user_port.save(person_dto.0.to_model()).to_dto()
    }
}
