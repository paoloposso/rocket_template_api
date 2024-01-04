use crate::user::{errors::CustomError, models::{user::User, use_case::user::GetUserResponse}, repository::UserDbTrait};

pub struct MockUserDB {}

#[async_trait]
impl UserDbTrait for MockUserDB {
    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError> {
        Ok(GetUserResponse {
            id: id.to_owned(),
            name: format!("{}'s name", id),
            email: format!("{}@example.com", id),
        })
    }

    async fn create(&self, _user: User) -> Result<String, CustomError> {
        Ok("new_user_id".into())
    }

    async fn delete(&self, _id: &str) -> Result<(), CustomError> {
        Ok(())
    }
}
