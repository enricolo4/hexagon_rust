use crate::rest::user::dto::{PersonDTO, PersonToDTO};

use domain::user::port::primary::UserPort;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use std::sync::Arc;

pub struct UserController {
    user_port: Box<dyn UserPort>
}

impl UserController {
    pub fn new(user_port: Box<dyn UserPort>) -> Self {
        return Self { user_port };
    }

    pub fn save(&self, person_dto: PersonDTO) -> PersonDTO {
        self.user_port.save(person_dto.to_model()).to_dto()
    }

    pub async fn health() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
}

pub async fn save(user_controller: actix_web::web::Data<Arc<UserController>>, person_dto: Json<PersonDTO>) -> impl Responder {
    let response = user_controller.save(person_dto.0);
    serde_json::to_string(&response)
}