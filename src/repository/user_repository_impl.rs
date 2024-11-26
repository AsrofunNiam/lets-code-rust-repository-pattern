use diesel::PgConnection;
use diesel::prelude::*;
use crate::repository::user_repository::UserRepository;
use crate::domain::user::User; 
use crate::schema::users::dsl::*;

pub struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    fn find_all(&self, connection: &mut PgConnection) -> Result<Vec<User>, String> {
        users
            .limit(10)
            .load::<User>(connection)
            .map_err(|err| format!("Error loading users: {}", err))
    }
    fn find_filter(&self, connection: &mut PgConnection) -> Result<Vec<User>, String> {
        users
            .load::<User>(connection)
            .map_err(|err| format!("Error loading users: {}", err))
    }

    fn update_user(&self, connection: &mut PgConnection, user_id: i32, new_name: &str, new_email: &str) -> Result<usize, String> {
        diesel::update(users.filter(id.eq(user_id)))
            .set((name.eq(new_name), email.eq(new_email)))
            .execute(connection)
            .map_err(|e| e.to_string())
    }
}
