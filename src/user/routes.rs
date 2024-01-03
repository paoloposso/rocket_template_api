use rocket::{get, State, http::Status};
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

use crate::user::service::UserServiceTrait;

use super::{models::user::User, errors::CustomError};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    id: String,
    name: String,
    email: String,
}

#[get("/user/<id>")]
pub fn get_by_id(id: String) -> Json<GetUserResponse> {
    Json(GetUserResponse {
        id,
        name: String::from("Dummy User"),
        email: String::from("dummy@example.com"),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    id: String,
}

#[post("/user", data = "<user>")]
pub async fn create(user_service: &State<Box<dyn UserServiceTrait>>, user: Json<CreateUserRequest>) 
    -> Result<Json<CreateUserResponse>, Status> {

    let new_id = "".to_string();
    
    let new_user = User {
        id: new_id,
        name: user.name.clone(),
        email: user.email.clone(),
        password: String::from(""),
    };

    let create_result = user_service.create(new_user).await;

    if let Err(err) = create_result {
        if err == CustomError::UserAlreadyExists {
            return Err(Status::Conflict);
        }
        return Err(Status::InternalServerError);
    } else {
        return Ok(Json(CreateUserResponse {
            id: create_result.unwrap(),
        }));
    }
}

#[delete("/user/<id>")]
pub async fn delete(user_service: &State<Box<dyn UserServiceTrait>>, id: String) -> Result<Status, Status> {
    user_service.delete(id).await.unwrap();
    Ok(Status::NoContent)
}
