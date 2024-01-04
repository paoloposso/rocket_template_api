use rocket::response::status;
use rocket::{get, State, http::Status};
use rocket::serde::json::Json;

use crate::user::models::use_case::user::CreateUserRequest;
use crate::user::service::UserServiceTrait;
use crate::core::api_response::ErrorResponse;
use crate::user::models::user::User;
use crate::user::errors::CustomError;

use super::models::use_case::user::{GetUserResponse, CreateUserResponse};

#[get("/user/<id>")]
pub async fn get_by_id(user_service: &State<Box<dyn UserServiceTrait>>, id: &str) -> Result<status::Custom<Json<GetUserResponse>>, status::Custom<Json<ErrorResponse>>> {

    let get_user_result = user_service.get_by_id(&id.to_owned()).await;

    if let Err(err) = get_user_result {
        match err {
            CustomError::UserNotFound => return Err(status::Custom(Status::NotFound, Json(ErrorResponse { message: "".to_string() }))),
            CustomError::GenericError(msg) => return Err(status::Custom(Status::InternalServerError, Json(ErrorResponse { message: format!("Generic error {}", msg) }))),
            _ => return Err(status::Custom(Status::InternalServerError, Json(ErrorResponse { message: format!("Unknown error {}", err.to_string()) }))),
        }
    }

    let user = get_user_result.unwrap();

    Ok(status::Custom(Status::Ok, Json(GetUserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
    })))
}

#[post("/user", data = "<user>")]
pub async fn create(user_service: &State<Box<dyn UserServiceTrait>>, user: Json<CreateUserRequest>) -> Result<status::Custom<Json<CreateUserResponse>>, status::Custom<Json<ErrorResponse>>> {
    
    let new_user = User {
        id: None,
        name: user.name.clone(),
        email: user.email.clone(),
        password: String::from(""),
    };

    let create_result = user_service.create(new_user).await;

    if let Err(err) = create_result {
        match err {
            CustomError::GenericError(msg) => return Err(status::Custom(Status::InternalServerError, Json(ErrorResponse { message: msg }))),
            CustomError::MissingFields(msg) => return Err(status::Custom(Status::BadRequest, Json(ErrorResponse { message: format!("The following properties are required: {}", msg) }))),
            _ => return Err(status::Custom(Status::InternalServerError, Json(ErrorResponse { message: err.to_string() }))),
        }
    }

    Ok(status::Custom(Status::Created, Json(CreateUserResponse {
        id: create_result.unwrap(),
    })))
}

#[delete("/user/<id>")]
pub async fn delete(user_service: &State<Box<dyn UserServiceTrait>>, id: &str) -> Result<status::Custom<()>, status::Custom<Json<ErrorResponse>>> {
    let delete_result = user_service.delete(id).await;

    if let Err(err) = delete_result {
        match err {
            CustomError::GenericError(msg) => return Err(status::Custom(Status::InternalServerError, Json(ErrorResponse { message: msg }))),
            CustomError::MissingFields(msg) => return Err(status::Custom(Status::BadRequest, Json(ErrorResponse { message: format!("The following properties are required: {}", msg) }))),
            _ => return Err(status::Custom(Status::InternalServerError, Json(ErrorResponse { message: err.to_string() }))),
        }
    }

    Ok(status::Custom(Status::Ok, ()))
}

#[cfg(test)]
mod e2e_tests {
    use crate::user::db::mongo::user_mongo::UserMongo;
    use crate::user::service::UserService;

    use super::*;
    use rocket::local::asynchronous::Client;
    use rocket::http::{Status, ContentType};
    use rocket::tokio;

    const MONGO_URI_TEST: &str = "mongodb://localhost:27018";

    #[tokio::test]
    async fn test_create_user() {
        let user_mongo = UserMongo::new(MONGO_URI_TEST).await.unwrap();

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

        assert_eq!(response.status(), Status::Created);

        let response_body: CreateUserResponse = serde_json::from_str(&
            response.into_string().await.unwrap()).unwrap();
        
        assert_ne!(response_body.id, "");
    }

    #[tokio::test]
    async fn test_create_user_bad_request() {
        let user_mongo = UserMongo::new(MONGO_URI_TEST).await.unwrap();
        let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(user_mongo)));

        let rocket = rocket::build()
            .manage(user_service)
            .mount("/", routes![create]);
        let client = Client::untracked(rocket).await.unwrap();

        let request = CreateUserRequest {
            name: "".into(),
            email: "test@example.com".into(),
            password: "password".into(),
        };

        let response = client
            .post("/user")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&request).unwrap())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::BadRequest);

        let response_body = response.into_string().await.unwrap();
        assert!(response_body.contains("The following properties are required"));
    }

    #[tokio::test]
    async fn test_get_user() {
        let user_mongo = UserMongo::new(MONGO_URI_TEST).await.unwrap();

        let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(user_mongo)));

        let rocket = rocket::build()
            .manage(user_service)
            .mount("/", routes![create])
            .mount("/", routes![get_by_id]);
        let client = Client::untracked(rocket).await.unwrap();

        let create_request = CreateUserRequest {
            name: "Test User".into(),
            email: "test@example.com".into(),
            password: "password".into(),
        };

        let create_response = client
            .post("/user")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&create_request).unwrap())
            .dispatch()
            .await;

        assert_eq!(create_response.status(), Status::Created);

        let create_response_body: CreateUserResponse =
            serde_json::from_str(&create_response.into_string().await.unwrap()).unwrap();
        
        let created_user_id = create_response_body.id;

        let get_response = client
            .get(format!("/user/{}", created_user_id))
            .dispatch()
            .await;

        assert_eq!(get_response.status(), Status::Ok);

        let get_response_body: GetUserResponse =
            serde_json::from_str(&get_response.into_string().await.unwrap()).unwrap();
        assert_eq!(get_response_body.id, created_user_id);
        assert_ne!(get_response_body.name, "");
        assert_ne!(get_response_body.email, "");
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let user_mongo = UserMongo::new(MONGO_URI_TEST).await.unwrap();

        let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(user_mongo)));

        let rocket = rocket::build()
            .manage(user_service)
            .mount("/", routes![get_by_id]);
        let client = Client::untracked(rocket).await.unwrap();

        let get_response = client
            .get("/user/6596be2aed81fa8f5b037c9f")
            .dispatch()
            .await;

        assert_eq!(get_response.status(), Status::NotFound);
    }

    #[tokio::test]
    async fn test_delete_user() {
        let user_mongo = UserMongo::new(MONGO_URI_TEST).await.unwrap();
        let user_service: Box<dyn UserServiceTrait> = Box::new(UserService::new(Box::new(user_mongo)));

        let rocket = rocket::build()
            .manage(user_service)
            .mount("/", routes![create])
            .mount("/", routes![delete]);
        let client = Client::untracked(rocket).await.expect("valid rocket instance");

        let create_request = CreateUserRequest {
            name: "Test User".into(),
            email: "test@example.com".into(),
            password: "password".into(),
        };

        let create_response = client
            .post("/user")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&create_request).unwrap())
            .dispatch()
            .await;

        assert_eq!(create_response.status(), Status::Created);

        let create_response_body: CreateUserResponse =
            serde_json::from_str(&create_response.into_string().await.unwrap()).unwrap();
        
        let created_user_id = create_response_body.id;

        let response = client.delete(format!("/user/{}", created_user_id)).dispatch().await;

        assert_eq!(response.status(), Status::Ok);
    }
}