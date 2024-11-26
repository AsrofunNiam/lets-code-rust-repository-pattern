// src/controller/user_controller.rs

use diesel::PgConnection;

use crate::domain::user::User; 



pub trait UserController {
    fn find_all(&self, connection: &mut PgConnection) -> Vec<User>; 
}
