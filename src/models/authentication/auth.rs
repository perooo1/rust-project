use crate::{
    errors::AuthError,
    models::user::{self, User},
    schema::users,
};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use std::{fmt, io::Error};
use validator::{Validate, ValidationError};

#[derive(Queryable, Debug, Deserialize, Validate)]
pub struct AuthUser {
    //#[validate(email)]
    pub email: String,
    //#[validate(length(min = 4), custom = "validate_user_password")]
    pub pass: String,
}

impl AuthUser {
    pub fn authenticate(
        conn: &PgConnection,
        email: String,
        password: String,
    ) -> Result<(User, String), AuthError> {
        let user = match users::table
            .filter(users::email.eq(&email))
            .load::<User>(conn)
        {
            Ok(mut users) => match users.pop() {
                Some(user) => user,
                _ => {
                    println!("Auth error: No user found with email: {}", &email);
                    return Err(AuthError);
                }
            },
            Err(e) => {
                println!("Authentication error: err getting user from db {:?}", e);
                return Err(AuthError);
            }
        };

        AuthUser::verify_password(password, &user)?;

        let token = user.generate_jwt();

        Ok((user, token))
    }

    fn verify_password(pwd: String, user: &User) -> Result<(), AuthError> {
        match bcrypt::verify(&pwd, &user.pass) {
            Ok(does_match) => {
                if does_match == true {
                    Ok(())
                } else {
                    println!("Auth error for bcrypt verification for : {}", &user.email);
                    Err(AuthError)
                }
            }
            Err(e) => {
                println!(
                    "Auth error for bcrypt verification for : {}, error msg: {}",
                    &user.email, e
                );
                Err(AuthError)
            }
        }
    }
}
