use crate::user::models::user::User;
use crate::user::errors::CustomError; // Add this line

pub trait UserDbTrait: Sync + Send {
    fn get_user(&self, id: String) -> Result<User, CustomError>;
}
