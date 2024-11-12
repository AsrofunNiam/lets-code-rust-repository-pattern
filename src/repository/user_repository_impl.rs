use crate::repository::user_repository::UserRepository;
use crate::domain::user::User;

pub struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    fn create_user(&self, name: String, email: String) -> Result<u64, String> { 
        println!("Creating user: {} with email {}", name, email);
        Ok(1)
    }

    fn get_user_by_id(&self, id: u64) -> Result<User, String> { 
        Ok(User::new(id, "arofun".to_string(), "asrofun@example.com".to_string()))
    }

    // fn update_user(&self, id: u64, name: String, email: String) -> Result<(), String> { 
    //     Ok(())
    // }

    // fn delete_user(&self, id: u64) -> Result<(), String> { 
    //     Ok(())
    // }
}
