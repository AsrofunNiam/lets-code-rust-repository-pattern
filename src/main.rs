mod domain;
mod repository;
mod service;
mod schema;
mod app;
mod controller; 
  
use controller::user_controller_impl::UserControllerImpl; 
use repository::user_repository_impl::UserRepositoryImpl; 
use controller::user_controller::UserController;
use service::user_service_impl::UserServiceImpl; 

fn main() {
    env_logger::init();
    let mut connection = app::establish_connection();
    let repository = UserRepositoryImpl;
    let controller = UserControllerImpl::new(UserServiceImpl::new(repository));

    match controller.find_filter(&mut connection) {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found.");
            } else {
                for user in users {
                    println!("ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
                }
            }
        }
        Err(err) => {
            eprintln!("Error fetching users: {}", err);
        }
    }

    match controller.find_all(&mut connection) {
        Ok(users) => {print!("{}", users.len())},
        Err(err) => {println!("Error fetching users: {}", err)}
        
    }

    // Update user with ID = 3
    match controller.update_user(&mut connection, 3, "Updated", "updatedagain@example.com") {
        Ok(message) => println!("{}", message),
        Err(err) => println!("Error: {}", err),
    }
}
