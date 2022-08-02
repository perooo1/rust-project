use crate::schema::users::{self};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{delete, pg::PgConnection};
use serde::{Deserialize, Serialize};
use validator::ValidationError;

use super::authentication::{self, jwt};

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
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub pass: String,
    pub is_admin: bool,
}

//TODO functions to get from databes ex get all, get by id...
impl User {
    pub fn get_all_users(conn: &PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        users::table.load::<Self>(conn)
    }

    pub fn get_user_by_id(
        conn: &PgConnection,
        id: String,
    ) -> Result<Option<User>, diesel::result::Error> {
        match users::table.filter(users::id.eq(id)).load::<User>(conn) {
            Ok(mut users) => Ok(users.pop()),
            Err(e) => Err(e),
        }
    }

    pub fn update_email(
        self,
        conn: &PgConnection,
        email: String,
    ) -> Result<(), diesel::result::Error> {
        match diesel::update(users::table.find(self.id))
            .set(users::email.eq(email))
            .get_result::<User>(conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn update_password(
        self,
        conn: &PgConnection,
        password: String,
    ) -> Result<(), diesel::result::Error> {
        let hash_password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(e) => {
                println!("Hashing password error: {:?}", e);
                return Err(diesel::result::Error::__Nonexhaustive);
            }
        };

        match diesel::update(users::table.find(self.id))
            .set(users::pass.eq(hash_password))
            .get_result::<User>(conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn delete_user(conn: &PgConnection, id: String) -> Result<usize, diesel::result::Error> {
        let count_deleted = delete(users::table.filter(users::id.eq(id))).execute(conn);
        count_deleted
    }

    pub fn generate_jwt(&self) -> String {
        jwt::generate(self)
    }

    pub fn create_user_from_jwt(claims: &authentication::jwt::Claims) -> Self {
        User {
            id: String::from(&claims.sub),
            first_name: String::from(&claims.first_name),
            last_name: String::from(&claims.last_name),
            email: String::from(&claims.email),
            pass: String::from(&claims.email),
            is_admin: claims.is_admin,
            created_at: claims.created_at,
            updated_at: claims.updated_at,
        }
    }
}

impl NewUser {
    //TODO functions for adding user to a database and associeted functions

    pub fn create_user(
        connection: &PgConnection,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
        is_admin: bool,
    ) -> Result<User, diesel::result::Error> {
        let is_email_valid = crate::validation::validate_email(&email);
        let is_pass_valid = crate::validation::validate_password(&password);

        if !is_email_valid || !is_pass_valid {
            //return Err(ValidationError::new("Error validating user email"));        //napravit svoj error
            println!("Error validating email: {} or password {}", email, password);
            return Err(diesel::result::Error::__Nonexhaustive); //ne ovo koristit za error!!
        } else {
            let hashed_password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
                //hashiranje izdvojit kasnije
                Ok(hashed) => hashed,
                Err(e) => {
                    println!("Hashing password error: {:?}", e);
                    return Err(diesel::result::Error::__Nonexhaustive);
                }
            };
            let new_user = Self {
                //mislim da i ovo
                first_name,
                last_name,
                email,
                pass: String::to_string(&hashed_password),
                is_admin,
            };

            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(connection)
            //.expect("Error adding new user")
        }
    }
}
