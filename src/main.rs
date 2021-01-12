use primary::rest::user::controller::{UserController, TestController};
use domain::user::use_case::{UserUseCase, TestUseCase};
use secondary::mysql::user::repository::{UserRepositoryAdapter, TestRepositoryAdapter};
use primary::rest::server_start;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref USER_CONTROLLER_CONTAINER: Arc<UserController> = {
        let _user_repository_port = UserRepositoryAdapter::new();
        let _user_use_case = UserUseCase::new(Arc::new(_user_repository_port));
        Arc::new(UserController::new(Arc::new(_user_use_case)))
    };
}

lazy_static! {
    static ref TEST_CONTROLLER_CONTAINER: Arc<TestController> = {
        let _test_repository_port = TestRepositoryAdapter::new();
        let _test_use_case = TestUseCase::new(Arc::new(_test_repository_port));
        Arc::new(TestController::new(Arc::new(_test_use_case)))
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let _user_controller: Arc<UserController> = USER_CONTROLLER_CONTAINER.clone();
    let _test_controller: Arc<TestController> = TEST_CONTROLLER_CONTAINER.clone();
    server_start(_user_controller, _test_controller).await
}