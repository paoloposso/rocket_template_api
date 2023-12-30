use super::models::user::User;

pub struct UserService;

pub trait UserServiceTrait {
     fn get_user(&self, id: String) -> User;
}

impl UserService {
    pub fn new() -> Self {
        UserService
    }
}

impl UserServiceTrait for UserService {
     fn get_user(&self, id: String) -> User {
        // Return a mock User for now
        User {
            id,
            name: "John Doe".to_string(),
            email: "johndoe@example.com".to_string(),
            password: "password".to_string(),
        }
    }
}