use diesel::PgConnection;
use crate::domain::user::User; 

pub trait UserController {
    fn find_all(&self, connection: &mut PgConnection) -> Result<Vec<User>, String>; 
    fn find_filter(&self, connection: &mut PgConnection) -> Result<Vec<User>, String>; 
    fn update_user(&self, connection: &mut PgConnection, user_id: i32, new_name: &str, new_email: &str) -> Result<String, String>;
}
