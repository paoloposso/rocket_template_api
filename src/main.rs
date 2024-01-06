#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod user {
    pub mod models {
        pub mod user;
        pub mod use_case;
    }
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
    pub mod api_response;
}

use user::db::mongo::user_mongo::UserMongo;
use dotenv::dotenv;
use std::env;
use user::service::{
    UserService, 
    UserServiceTrait};

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_URI not found in environment variables");
    let mongo_db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME not found in environment variables");
    let mongo_repo = UserMongo::new(&mongo_uri, &mongo_db_name).await.unwrap();
    let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(mongo_repo)));

    rocket::build()
        .manage(user_service)
        .mount("/", routes![user::routes::get_by_id])
        .mount("/", routes![user::routes::create])
        .mount("/", routes![user::routes::delete])
}
