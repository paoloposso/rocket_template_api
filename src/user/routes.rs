use rocket::get;
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[get("/users/<id>")]
pub fn get_by_id(id: String) -> Json<GetUserResponse> {
    Json(GetUserResponse {
        id,
        name: String::from("Dummy User"),
        email: String::from("dummy@example.com"),
    })
}
