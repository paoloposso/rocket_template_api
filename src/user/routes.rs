use rocket::{get, State, http::Status};
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

use crate::user::service::UserServiceTrait;
use crate::core::api_responses::{ApiResult, ApiNoContentResult, ErrorResponse};
use crate::user::models::user::User;
use crate::user::errors::CustomError;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    id: String,
    name: String,
    email: String,
}

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

#[cfg(test)]
mod e2e_tests {
    use crate::user::db::mock::user_db_mock::MockUserDB;
    use crate::user::db::mongo::user_mongo::UserMongo;
    use crate::user::service::UserService;

    use super::*;
    use rocket::local::asynchronous::Client;
    use rocket::http::{Status, ContentType};
    use rocket::tokio;

    #[tokio::test]
    async fn test_create_user() {
        let user_mongo = UserMongo::new().await.unwrap();

        let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(user_mongo)));

        let rocket = rocket::build()
            .manage(user_service)
            .mount("/", routes![create]);
        let client = Client::untracked(rocket).await.unwrap();

        let request = CreateUserRequest {
            name: "Test User".into(),
            email: "test@example.com".into(),
            password: "password".into(),
        };

        let response = client.post("/user")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&request).unwrap())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_get_user() {
        let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(MockUserDB {})));

        let rocket = rocket::build()
            .manage(user_service)
            .mount("/", routes![get_by_id]);
        let client = Client::untracked(rocket).await.unwrap();

        let response = client.get("/user/123").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(MockUserDB {})));

        let rocket = rocket::build()
            .manage(user_service)
            .mount("/", routes![delete]);
        let client = Client::untracked(rocket).await.expect("valid rocket instance");

        let response = client.delete("/user/123").dispatch().await;

        assert_eq!(response.status(), Status::NoContent);
    }
}