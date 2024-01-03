use super::{models::user::User, errors::CustomError, repository::UserDbTrait};

pub struct UserService {
    user_db: Box<dyn UserDbTrait>,
}

impl UserService {
    pub fn new(user_db: Box<dyn UserDbTrait>) -> Self {
        UserService { user_db }
    }
}

#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn get_by_id(&self, id: String) -> Result<User, CustomError>;
    async fn create(&self, user: User) -> Result<String, CustomError>;
    async fn delete(&self, id: String) -> Result<(), CustomError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_by_id(&self, id: String) -> Result<User, CustomError> {
        self.user_db.get_by_id(id).await
    }

    async fn create(&self, user: User) -> Result<String, CustomError> {
        self.user_db.create(user).await
    }

    async fn delete(&self, id: String) -> Result<(), CustomError> {
        self.user_db.delete(id).await
    }
}
