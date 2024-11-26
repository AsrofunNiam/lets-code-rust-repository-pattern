use diesel::prelude::*;
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

    fn find_all (&self, connection: &mut PgConnection) -> Result<Vec<User>, String>{
        self.user_repository.find_all(connection)
    }  
    fn find_filter (&self, connection: &mut PgConnection) -> Result<Vec<User>, String>{ 
         let raw_users = self.user_repository.find_filter(connection)?; 
         let mut filtered_users = Vec::new(); 
         for user in raw_users {
             if user.id > 3 {
                 filtered_users.push(user);
             }
         } 
         Ok(filtered_users)
    }  

    fn update_user(&self, connection: &mut PgConnection, user_id: i32, new_name: &str, new_email: &str) -> Result<String, String> {
        if new_name.is_empty() || new_email.is_empty() {
            return Err("Name and email cannot be empty.".to_string());
        }

        match self.user_repository.update_user(connection, user_id, new_name, new_email) {
            Ok(rows_updated) if rows_updated > 0 => Ok(format!("Successfully updated {} user(s).", rows_updated)),
            Ok(_) => Err("No user found with the given ID.".to_string()),
            Err(err) => Err(format!("Failed to update user: {}", err)),
        }
    }
}
