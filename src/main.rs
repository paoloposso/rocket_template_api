#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod routes;
mod user;
mod user {
    pub mod models;
    pub mod routes;
    pub mod service;
}

use user::service::{UserService, UserServiceTrait};

#[launch]
async fn rocket() -> _ {
    let user_service = UserService::new();

    rocket::build()
        .manage(Box::new(user_service) as Box<dyn UserServiceTrait + Send + Sync>)
        .mount("/", routes![user::routes::get])
}
