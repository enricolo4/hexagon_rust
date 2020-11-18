use crate::rest::user::dto::{PersonDTO, PersonToDTO};

use domain::user::port::primary::UserPort;
use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use std::sync::Arc;

pub struct UserController {
    user_port: Arc<dyn UserPort>
}

impl UserController {
    pub fn new(user_port: Arc<dyn UserPort>) -> Self { Self { user_port } }

    pub fn save(user_controller: actix_web::web::Data<Arc<UserController>>, person_dto: Json<PersonDTO>) -> PersonDTO {
        user_controller.save(person_dto.0.to_model()).to_dto()
    }
}

// pub async fn save(user_controller: actix_web::web::Data<Arc<UserController>>, person_dto: Json<PersonDTO>) -> impl Responder {
//     println!("{:?}", person_dto.0);
//     let response = user_controller.save(person_dto);
//     Json(response)
//     // serde_json::to_string(&response)
//     // Json({})
// }