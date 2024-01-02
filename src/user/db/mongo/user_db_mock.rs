use crate::user::{db::traits::user_db_trait::UserDbTrait, errors::CustomError, models::user::User};

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

    async fn create(&self, user: User) -> Result<User, CustomError> {
        // Implementation for create method goes here
        unimplemented!()
    }

    async fn update(&self, id: String, user: User) -> Result<User, CustomError> {
        // Implementation for update method goes here
        unimplemented!()
    }

    async fn delete(&self, id: String) -> Result<(), CustomError> {
        // Implementation for delete method goes here
        unimplemented!()
    }
}
    