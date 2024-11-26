// src/service/user_service.rs

use diesel::PgConnection; 

use crate::domain::user::User;

pub trait UserService {
    fn find_all(&self, connection: &mut PgConnection) -> Vec<User>;
    // fn create_user(&self, name: String, email: String, id: u64) -> Result<u64, String>;
    // fn get_user_by_id(&self, id: u64) -> Result<User, String>;
    // fn update_user(&self, id: u64, name: String, email: String) -> Result<(), String>;
    // fn delete_user(&self, id: u64) -> Result<(), String>;
}
