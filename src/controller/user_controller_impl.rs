use diesel::prelude::*; 

use crate::{domain::user::User, service::user_service::UserService};

 

use crate::controller::user_controller::UserController; 


pub struct UserControllerImpl <S : UserService> {
    service: S,
}

impl <S : UserService> UserControllerImpl<S> {
    pub fn new(service:  S) -> Self {
        UserControllerImpl { service }
    }
}


impl <S : UserService> UserController for UserControllerImpl<S> {
    fn find_all(&self, connection: &mut PgConnection) -> Vec<User> { 
        match self.service.find_all(connection) {
            users if !users.is_empty() => users, 
            _ => Vec::new()
        }
    }

     
}