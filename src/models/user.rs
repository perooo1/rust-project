use crate::custom_errors::app_error::AppError;
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::authentication::{self, jwt};

///Struct representing a user in postgres databse
#[derive(Queryable, PartialEq, Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

///Struct used for creating a new user in postgres database
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
    pub is_admin: bool,
}

impl User {
    ///Function for returning all users from postgres database
    /// # Returns
    /// ## Ok
    /// - Vector of registered users in database: Vec<[User]>
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    pub fn get_all_users(conn: &PgConnection) -> Result<Vec<Self>, AppError> {
        users::table
            .load::<Self>(conn)
            .map_err(|_| AppError::InternalError)
    }
    ///Function for getting a user with matching id from postgres database
    /// # Returns
    /// ## Ok
    /// - User in database: user: [User]
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Not Found error
    pub fn get_user_by_id(conn: &PgConnection, id: &String) -> Result<Option<User>, AppError> {
        match users::table.filter(users::id.eq(id)).load::<User>(conn) {
            Ok(mut users) => Ok(users.pop()),
            Err(e) => {
                println!("{}", e.to_string());
                Err(AppError::NotFound)
            }
        }
    }
    ///Function for deleting an user with matching id from postgres database
    /// # Returns
    /// ## Ok
    /// - Number of rows affected: count_deleted: [usize]
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Not Found error
    pub fn delete_user(conn: &PgConnection, id: String) -> Result<usize, AppError> {
        match diesel::update(users::table.find(id))
            .set(users::is_deleted.eq(true))
            .execute(conn)
        {
            Ok(count_deleted) => {
                if count_deleted == 0 {
                    return Err(AppError::NotFound);
                }
                println!("Update user deletion deleted {} rows", count_deleted);
                Ok(count_deleted)
            }
            Err(_) => {
                println!("Error setting user deleted to false");
                Err(AppError::InternalError)
            }
        }
    }
    ///Function used for generating a jwt token for user
    pub fn generate_jwt(&self) -> String {
        jwt::generate(self)
    }
}

impl NewUser {
    ///Function for creating a new user in postgres database
    /// # Returns
    /// ## Ok
    /// - user: [User]
    /// ## Error
    /// - error: [AppError]
    /// - User Creation error
    /// - Invalid credentials error
    /// - Password hash error
    /// - Internal server error
    pub fn create_user(
        connection: &PgConnection,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        is_admin: bool,
    ) -> Result<User, AppError> {
        let is_email_valid = crate::validation::user_validation::validate_email(&email);
        let is_pass_valid = crate::validation::user_validation::validate_password(&password);

        if !is_email_valid || !is_pass_valid {
            println!("Error validating email: {} or password {}", email, password);
            return Err(AppError::InvalidCredentials);
        } else {
            let hashed_password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
                Ok(hashed) => hashed,
                Err(e) => {
                    println!("Hashing password error: {:?}", e);
                    return Err(AppError::PasswordHashError);
                }
            };
            let new_user = Self {
                first_name,
                last_name,
                email,
                pass: String::to_string(&hashed_password),
                is_admin,
            };

            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(connection)
                .map_err(|_| AppError::UserCreationError)
        }
    }
}
