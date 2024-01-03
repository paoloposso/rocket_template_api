use crate::user::models::user::User;
use crate::user::errors::CustomError;

#[async_trait]
pub trait UserDbTrait: Sync + Send {
    async fn get_by_id(&self, id: String) -> Result<User, CustomError>;
    async fn create(&self, user: User) -> Result<String, CustomError>;
    async fn delete(&self, id: String) -> Result<(), CustomError>;
}
