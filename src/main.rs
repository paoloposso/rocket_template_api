#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod user {
    pub mod models;
    pub mod routes;
    pub mod service;
    pub mod errors;
    pub mod db {
        pub mod traits;
        pub mod mongo;
    }
}

use std::sync::Arc;

use user::service::{UserService, UserServiceTrait};
use user::db::mongo::user_db_mock::MockUserDB;

#[launch]
async fn rocket() -> _ {
    let user_service = Arc::new(UserService::new(Arc::new(MockUserDB {}))) as Arc<dyn UserServiceTrait + Send + Sync>;

    rocket::build()
        .manage(user_service)
        .mount("/", routes![user::routes::get_by_id])
}
