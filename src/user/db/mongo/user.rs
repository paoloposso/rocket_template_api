use crate::user::{db::traits::user::UserDbTrait, errors::CustomError, models::user::User};

pub struct MockUserDB {}

#[async_trait]
impl UserDbTrait for MockUserDB {
    fn get_user(&self, id: String) -> Result<User, CustomError> {
        Ok(User {
            id: id.clone(),
            name: format!("{}'s name", id),
            email: format!("{}@example.com", id),
            password: format!("{}'s password", id),
        })
    }
}
