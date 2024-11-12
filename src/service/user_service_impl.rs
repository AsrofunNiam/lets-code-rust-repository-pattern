use crate::service::user_service::UserService;
use crate::repository::user_repository::UserRepository;
use crate::domain::user::User;

pub struct UserServiceImpl<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserServiceImpl<R> {
    pub fn new(user_repository: R) -> Self {
        UserServiceImpl { user_repository }
    }
}

impl<R: UserRepository> UserService for UserServiceImpl<R> {
    fn create_user(&self, name: String, email: String, _id: u64) -> Result<u64, String> {
        self.user_repository.create_user(name, email)
    }

    fn get_user_by_id(&self, id: u64) -> Result<User, String> {
        self.user_repository.get_user_by_id(id)
    }

    // fn update_user(&self, id: u64, name: String, email: String) -> Result<(), String> {
    //     self.user_repository.update_user(id, name, email)
    // }

    // fn delete_user(&self, id: u64) -> Result<(), String> {
    //     self.user_repository.delete_user(id)
    // }
}
