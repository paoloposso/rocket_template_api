use crate::user::{errors::CustomError, models::user::User, repository::UserDbTrait};

pub struct MockUserDB {}

#[async_trait]
impl UserDbTrait for MockUserDB {
    async fn get_by_id(&self, id: String) -> Result<User, CustomError> {
        Ok(User {
            id: id.clone(),
            name: format!("{}'s name", id),
            email: format!("{}@example.com", id),
            password: format!("{}'s password", id),
        })
    }

    async fn create(&self, user: User) -> Result<String, CustomError> {
        Ok("new_user_id".to_string())
    }

    async fn update(&self, id: String, user: User) -> Result<(), CustomError> {
        Ok(())
    }

    async fn delete(&self, id: String) -> Result<(), CustomError> {
        Ok(())
    }
}
    