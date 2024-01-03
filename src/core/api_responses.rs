use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

pub type ApiResult<T> = Result<Json<T>, Json<ErrorResponse>>;
pub type ApiNoContentResult = Result<Status, Json<ErrorResponse>>;