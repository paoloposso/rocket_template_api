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

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

type ApiResult<T> = Result<Json<T>, Json<ErrorResponse>>;
type ApiNoContentResult = Result<Status, Json<ErrorResponse>>;

#[get("/user/<id>")]
pub async fn get_by_id(user_service: &State<Box<dyn UserServiceTrait>>, id: String) -> ApiResult<GetUserResponse> {

    let get_user_result = user_service.get_by_id(id.clone()).await;

    if let Err(err) = get_user_result {
        match err {
            CustomError::UserNotFound => return Err(Json(ErrorResponse { status: Status::NotFound, message: "".to_string() })),
            CustomError::GenericError(msg) => return Err(Json(ErrorResponse { status: Status::InternalServerError, message: msg.to_string() })),
            _ => return Err(Json(ErrorResponse { status: Status::InternalServerError, message: "".to_string() })),
        }
    }

    let user = get_user_result.unwrap();

    Ok(Json(GetUserResponse {
        id,
        name: user.name,
        email: user.email,
    }))
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
pub async fn create(user_service: &State<Box<dyn UserServiceTrait>>, user: Json<CreateUserRequest>) -> ApiResult<CreateUserResponse> {
    let new_id = "".to_string();
    
    let new_user = User {
        id: new_id,
        name: user.name.clone(),
        email: user.email.clone(),
        password: String::from(""),
    };

    let create_result = user_service.create(new_user).await;

    if let Err(err) = create_result {
        match err {
            CustomError::GenericError(msg) => return Err(Json(ErrorResponse { status: Status::InternalServerError, message: msg.to_string() })),
            _ => return Err(Json(ErrorResponse { status: Status::InternalServerError, message: "".to_string() })),
        }
    }

    Ok(Json(CreateUserResponse {
        id: "".to_string(),
    }))
}

#[delete("/user/<id>")]
pub async fn delete(user_service: &State<Box<dyn UserServiceTrait>>, id: String) -> ApiNoContentResult {
    user_service.delete(id).await.unwrap();
    Ok(Status::NoContent)
}
