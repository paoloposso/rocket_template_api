use crate::user::models::user::User;
use crate::user::errors::CustomError;

use super::models::use_case::user::GetUserResponse;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserDbTrait: Sync + Send {
    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError>;
    async fn create(&self, user: User) -> Result<String, CustomError>;
    async fn delete(&self, id: &str) -> Result<(), CustomError>;
}
