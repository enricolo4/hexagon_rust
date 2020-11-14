use domain::user::use_case::UserUseCase;
use secondary::mysql::user::repository::UserRepositoryAdapter;
use primary::rest::user::controller::{UserController, save};
use actix_web::{HttpServer, App, web};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// static ADAPTER: Lazy<Mutex<UserRepositoryAdapter>> = Lazy::new(|| {
//     Mutex::new(UserRepositoryAdapter::new())
// });
//
// static USER_USE_CASE: Lazy<Mutex<UserUseCase>> = Lazy::new(|| {
//     let _adapter = ADAPTER.lock().unwrap();
//     Mutex::new(UserUseCase::new(Box::new(_adapter)))
// });

static USER_CONTROLLER: Lazy<Mutex<UserController>> = Lazy::new(|| {
    let _adapter = UserRepositoryAdapter::new();
    let _user_use_case = UserUseCase::new(Box::new(_adapter));
    Mutex::new(UserController::new(Box::new(_user_use_case)))
});

// fn container() -> Arc<UserController> {
//     let _adapter = UserRepositoryAdapter::new();
//     let _user_use_case = UserUseCase::new(Box::new(_adapter));
//     let _user_controller = UserController::new(Box::new(_user_use_case));
//     Arc::new(_user_controller)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let _person_dto = PersonDTO::new(
    //         "Testando 123".to_string(),
    //         34,
    //         "12345678978".to_string(),
    //         "testando_123@gmail.com".to_string()
    //     );

    let user_controller = USER_CONTROLLER.lock().unwrap();

    // println!("{:?} Salvo", _user_controller.save(_person_dto).await);
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(UserController::health))
            .route("/health", web::post().to(save))
    })

        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
