// src/main.rs

mod domain;
mod repository;
mod service;
mod schema;
mod app; 

use diesel::{connection, prelude::*};
use domain::user::User;
use dotenvy::dotenv;
use schema::users;
use std::env; 
// use crate::schema::users;
// use s::users::dsl::*;
use diesel::prelude::*;
use crate::schema::users::dsl::*; 



use repository::product_repository_impl::ProductRepositoryImpl;
use repository::user_repository_impl::UserRepositoryImpl;
use service::product_service::ProductService;
use service::product_service_impl::ProductServiceImpl;
use service::user_service_impl::UserServiceImpl;
use service::user_service::UserService;

fn main() {
    let user_repository = UserRepositoryImpl;
    let user_service = UserServiceImpl::new(user_repository);

    // let product_repository = ProductRepositoryImpl;
    // let product_service = ProductServiceImpl::new(product_repository);

    // // create a user
    // match user_service.create_user("asrofun".to_string(), "asrofune@example.com".to_string(), 1) {
    //     Ok(user_id) => println!("User created with ID: {}", user_id),
    //     Err(err) => println!("Error: {}", err),
    // }

    // match user_service.get_user_by_id(1) {
    //     Ok(user) => println!("User found: {} with email {} and wid: {}", user.name, user.email, user.id.to_string()),
    //     Err(err) => println!("Error: {}", err),
    // }

    // // create a product
    // match product_service.create_product("asrofun".to_string(), "asrofune@example.com".to_string(), "BIOSAN".to_string()) {
    //     Ok(user_id) => println!("Product created with ID: {}", user_id),
    //     Err(err) => println!("Error: {}", err),
    // }

    // match product_service.get_product_by_id("BIOSAN".to_string()) {
    //     Ok(product) => println!("Product found: {} description {} and product id: {}", product.name, product.description, product.id),
    //     Err(err) => println!("Error: {}", err),
    // }

    let mut connection = app::establish_connection();
    
    // let user_list = get_users(&mut connection); 
    // for user in user_list {
    //     println!("ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
    // }
 
//   find all data users 
    let test = user_service.find_all(&mut connection);

    for user in test {
        println!("ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
    }

}


// pub fn get_users(connection: &mut PgConnection) -> Vec<User> {
//     users
//         .limit(10)
//         .load::<User>(connection)
//         .expect("Error loading users")
// }