use crate::rest::user::controller::{UserController};
use actix_web::{HttpServer, web, App};
use actix_web::web::{Data, Json};
use std::sync::Arc;
use crate::rest::user::dto::PersonDTO;

pub mod user;

pub async fn server_start(data: Arc<UserController>) -> std::io::Result<()> {
    HttpServer::new( move|| {
        App::new()
            .data(data.clone())
            .route("/person", web::post().to(UserController::save))
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}