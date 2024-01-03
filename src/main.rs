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
        pub mod mongo;
    }
}

pub mod core {
    pub mod api_responses;
}

use user::db::mongo::user_mongo::UserMongo;
use user::service::{
    UserService, 
    UserServiceTrait};

#[launch]
async fn rocket() -> _ {
    let mongo_repo = UserMongo::new().await.unwrap();
    let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(mongo_repo)));

    rocket::build()
        .manage(user_service)
        .mount("/", routes![user::routes::get_by_id])
        .mount("/", routes![user::routes::create])
}
