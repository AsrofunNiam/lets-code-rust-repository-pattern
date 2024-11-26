use diesel::prelude::*;
use crate::{domain::user::User, service::user_service::UserService};
use crate::controller::user_controller::UserController;

pub struct UserControllerImpl<S: UserService> {
    service: S,
}

impl<S: UserService> UserControllerImpl<S> {
    pub fn new(service: S) -> Self {
        UserControllerImpl { service }
    }
}

impl<S: UserService> UserController for UserControllerImpl<S> {
    fn find_all(&self, connection: &mut PgConnection) -> Result<Vec<User>, String> { 
        self.service.find_all(connection)
    }
    fn find_filter(&self, connection: &mut PgConnection) -> Result<Vec<User>, String> { 
        self.service.find_filter(connection)
    }

    fn update_user(&self, connection: &mut PgConnection, user_id: i32, new_name: &str, new_email: &str) -> Result<String, String> {
        self.service.update_user(connection, user_id, new_name, new_email)
    }
}
