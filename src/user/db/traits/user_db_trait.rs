use crate::user::models::user::User;
use crate::user::errors::CustomError; // Add this line

#[async_trait]
pub trait UserDbTrait: Sync + Send {
    async fn get_by_id(&self, id: String) -> Result<User, CustomError>;
    async fn create(&self, user: User) -> Result<String, CustomError>;
    async fn update(&self, id: String, user: User) -> Result<(), CustomError>;
    async fn delete(&self, id: String) -> Result<(), CustomError>;
}
