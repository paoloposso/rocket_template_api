
#[derive(Debug)]
#[derive(PartialEq)]
pub enum CustomError {
    UserNotFound,
    UserAlreadyExists,
    GenericError(String)
}
