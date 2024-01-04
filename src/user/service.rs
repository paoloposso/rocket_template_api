use super::{models::{user::User, use_case::user::GetUserResponse}, errors::CustomError, repository::UserDbTrait};

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
    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError>;
    async fn create(&self, user: User) -> Result<String, CustomError>;
    async fn delete(&self, id: &str) -> Result<(), CustomError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError> {
        self.user_db.get_by_id(id).await
    }

    async fn create(&self, user: User) -> Result<String, CustomError> {
        let mut missing_properties: Vec<&str> = vec![];

        if user.name.is_empty() {
            missing_properties.push("name");
        }
        if user.email.is_empty() {
            missing_properties.push("email");
        }

        if !missing_properties.is_empty() {
            return Err(CustomError::MissingFields(
                missing_properties.join(", ").to_string(),
            ));
        }

        self.user_db.create(user).await
    }

    async fn delete(&self, id: &str) -> Result<(), CustomError> {
        self.user_db.delete(id).await
    }
}
