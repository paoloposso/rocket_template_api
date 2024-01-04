use crate::user::{errors::CustomError, models::user::User, repository::UserDbTrait};

pub struct MockUserDB {}

#[async_trait]
impl UserDbTrait for MockUserDB {
    async fn get_by_id(&self, id: &str) -> Result<User, CustomError> {
        Ok(User {
            id: Some(id.to_owned()),
            name: format!("{}'s name", id),
            email: format!("{}@example.com", id),
            password: format!("{}'s password", id),
        })
    }

    async fn create(&self, _user: User) -> Result<String, CustomError> {
        Ok("new_user_id".to_string())
    }

    async fn delete(&self, _id: &str) -> Result<(), CustomError> {
        Ok(())
    }
}
    