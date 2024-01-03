#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod user {
    pub mod models;
    pub mod routes;
    pub mod service;
    pub mod errors;
    pub mod repository;
    pub mod db {
        pub mod mock;
    }
}

pub mod core {
    pub mod api_responses;
}

use user::service::{
    UserService, 
    UserServiceTrait};
use user::db::mock::user_db_mock::MockUserDB;

#[launch]
async fn rocket() -> _ {
    let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(MockUserDB {})));

    rocket::build()
        .manage(user_service)
        .mount("/", routes![user::routes::get_by_id])
        .mount("/", routes![user::routes::create])
}
