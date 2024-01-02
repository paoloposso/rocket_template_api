use std::sync::Arc;

use super::{models::user::User, db::traits::user_db_trait::UserDbTrait, errors::CustomError};

pub struct UserService {
    user_db: Arc<dyn UserDbTrait>,
}

impl UserService {
    pub fn new(user_db: Arc<dyn UserDbTrait>) -> Self {
        UserService { user_db }
    }
}

#[async_trait]
pub trait UserServiceTrait {
    async fn get_by_id(&self, id: String) -> Result<User, CustomError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_by_id(&self, id: String) -> Result<User, CustomError> {
        self.user_db.get_by_id(id).await
    }
}
