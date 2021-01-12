use crate::rest::user::controller::{UserController, TestController};
use actix_web::{HttpServer, web, App, Responder};
use std::sync::Arc;

pub mod user;


pub async fn health() -> impl Responder { "hello world" }

pub async fn server_start(
    user_controller: Arc<UserController>,
    test_controller: Arc<TestController>
) -> std::io::Result<()> {
    HttpServer::new( move|| {
        App::new()
            .data(user_controller.clone())
            .route("/person", web::post().to(UserController::save))
            .route("/health", web::get().to(health))
            .data(test_controller.clone())
            .route("/test", web::post().to(TestController::save))
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}