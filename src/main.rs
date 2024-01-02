
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

use user::service::UserService;
use user::db::mongo::user::MockUserDB;

#[launch]
async fn rocket() -> _ {
    let user_service = Box::new(UserService::new(Box::new(MockUserDB {})));

    rocket::build()
        .manage(user_service)
        .mount("/", routes![user::routes::get])
}
