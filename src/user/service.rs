use super::{models::user::User, db::traits::user::UserDbTrait, errors::CustomError};

pub struct UserService {
    user_db: Box<dyn UserDbTrait>,
}

impl UserService {
    pub fn new(user_db: Box<dyn UserDbTrait>) -> Self {
        UserService { user_db }
    }
}

pub trait UserServiceTrait {
    fn get_user(&self, id: String) -> impl std::future::Future<Output = Result<User, CustomError>> + Send;
}

impl UserServiceTrait for UserService {
    async fn get_user(&self, id: String) -> Result<User, CustomError> {
        self.user_db.get_user(id)
    }
}
