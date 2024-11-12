use crate::domain::user::User;

pub trait UserRepository {
    fn create_user(&self, name: String, email: String) -> Result<u64, String>;
    fn get_user_by_id(&self, id: u64) -> Result<User, String>;
    // fn update_user(&self, id: u64, name: String, email: String) -> Result<(), String>;
    // fn delete_user(&self, id: u64) -> Result<(), String>;
}
