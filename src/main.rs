// src/main.rs

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

    let mut connection = app::establish_connection(); 
     let repository = UserRepositoryImpl; 

    let controller = UserControllerImpl::new(UserServiceImpl::new(repository));

    let data_controller =   controller.find_all(&mut connection);

    for user in data_controller {
        println!("ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
    }
 

} 