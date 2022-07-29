use crate::schema::users::{self};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

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
}

impl NewUser {
    //TODO functions for adding user to a database and associeted functions

    pub fn create_user(
        connection: &PgConnection,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> Result<User, diesel::result::Error> {
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
            is_admin: false,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(connection)
        //.expect("Error adding new user")
    }

    pub fn create_admin(
        connection: &PgConnection,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> Result<User, diesel::result::Error> {
        let hashed_password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(e) => {
                println!("Hashing password error: {:?}", e);
                return Err(diesel::result::Error::__Nonexhaustive);
            }
        };
        let new_admin = Self {
            first_name,
            last_name,
            email,
            pass: String::to_string(&hashed_password),
            is_admin: true,
        };

        diesel::insert_into(users::table)
            .values(&new_admin)
            .get_result(connection)
        //.expect("Error adding new admin")
    }
}
