#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod routes;
mod user {
    pub mod models;
    pub mod routes;
}

use routes::hello::hello;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/", routes![user::routes::get])
}
