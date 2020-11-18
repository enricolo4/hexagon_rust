use primary::rest::user::controller::{UserController};
use domain::user::use_case::UserUseCase;
use secondary::mysql::user::repository::UserRepositoryAdapter;
use primary::rest::server_start;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref CONTAINER: Arc<UserController> = {
        let _user_repository_port = UserRepositoryAdapter::new();
        let _user_use_case = UserUseCase::new(Arc::new(_user_repository_port));
        Arc::new(UserController::new(Arc::new(_user_use_case)))
    };
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let bla: Arc<UserController> = CONTAINER.clone();
    server_start(bla).await
}