use diesel::prelude::*;
use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String, 
    pub email: String,  
}


impl User {
    pub fn new(id: i32, name: String, email: String) -> Self {
        User { id,name,  email }
    }
}
