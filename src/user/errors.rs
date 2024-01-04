use core::fmt;
use std::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum CustomError {
    UserNotFound,
    UserAlreadyExists,
    GenericError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::UserNotFound => write!(f, "User not found"),
            CustomError::UserAlreadyExists => write!(f, "User already exists"),
            CustomError::GenericError(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

impl From<mongodb::error::Error> for CustomError {
    fn from(err: mongodb::error::Error) -> Self {
        CustomError::GenericError(err.to_string())
    }
}

impl Error for CustomError {}