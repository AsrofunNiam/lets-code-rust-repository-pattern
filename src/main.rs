// src/main.rs

mod domain;
mod repository;
mod service;

use repository::user_repository_impl::UserRepositoryImpl;
use service::user_service_impl::UserServiceImpl;
use service::user_service::UserService;

fn main() {
    let user_repository = UserRepositoryImpl;
    let user_service = UserServiceImpl::new(user_repository);

    match user_service.create_user("asrofun".to_string(), "asrofune@example.com".to_string(), 1) {
        Ok(user_id) => println!("User created with ID: {}", user_id),
        Err(err) => println!("Error: {}", err),
    }

    match user_service.get_user_by_id(1) {
        Ok(user) => println!("User found: {} with email {} and wid: {}", user.name, user.email, user.id.to_string()),
        Err(err) => println!("Error: {}", err),
    }
}
