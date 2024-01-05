use super::{models::{user::User, use_case::user::{GetUserResponse, CreateUserRequest}}, errors::CustomError, repository::UserDbTrait};
use bcrypt::{hash, DEFAULT_COST};

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
    async fn create(&self, new_user: CreateUserRequest) -> Result<String, CustomError>;
    async fn delete(&self, id: &str) -> Result<(), CustomError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_by_id(&self, id: &str) -> Result<GetUserResponse, CustomError> {
        self.user_db.get_by_id(id).await
    }

    async fn create(&self, new_user: CreateUserRequest) -> Result<String, CustomError> {
        let mut missing_properties: Vec<&str> = vec![];

        if new_user.name.is_empty() {
            missing_properties.push("name");
        }
        if new_user.email.is_empty() {
            missing_properties.push("email");
        }
        if new_user.plain_password.is_empty() {
            missing_properties.push("password");
        }

        if !missing_properties.is_empty() {
            return Err(CustomError::MissingFields(
                missing_properties.join(", ").to_string(),
            ));
        }

        let hashed_password = hash(&new_user.plain_password, DEFAULT_COST)
            .map_err(|err| CustomError::GenericError(format!("Hashing error: {}", err)))?;

        let user = User {
            password: hashed_password,
            name: new_user.name,
            id: None,
            email: new_user.email,
        };

        self.user_db.create(user).await
    }

    async fn delete(&self, id: &str) -> Result<(), CustomError> {
        self.user_db.delete(id).await
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::user::repository::MockUserDbTrait;

    use super::*;
    use rocket::tokio;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_create_user() {
        let mut mock_db = MockUserDbTrait::new();
        mock_db
            .expect_create()
            .with(always())
            .returning(|_| Ok("12344321".to_string()));
        let user_service = UserService { user_db: Box::new(mock_db) };

        let test_user = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@test.com".to_string(),
            plain_password: "1234".to_string(),
        };

        let result = user_service.create(test_user).await;

        assert_eq!(result.unwrap(), "12344321");
    }

    #[tokio::test]
    async fn test_create_user_missing_fields() {
        let mock_db = MockUserDbTrait::new();
        
        let user_service = UserService { user_db: Box::new(mock_db) };

        let test_user = CreateUserRequest {
            name: "Test User".to_string(),
            email: "".to_string(),
            plain_password: "1234".to_string(),
        };

        let result = user_service.create(test_user).await;

        assert_eq!(
            result.unwrap_err(),
            CustomError::MissingFields("email".to_string())
        );
    }
}