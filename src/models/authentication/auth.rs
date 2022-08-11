use crate::{custom_errors::app_error::AppError, models::user::User, schema::users};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

///Struct used for user authentication purposes
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct AuthUser {
    pub email: String,
    pub pass: String,
}

impl AuthUser {
    ///Function used to authenticate user if registered
    /// # Returns
    /// ## Ok
    /// - Touple of (registered) user struct and generated jwt token: (user: [User], token: [String])
    /// ## Error
    /// - error: [AppError]
    pub fn authenticate(
        conn: &PgConnection,
        email: String,
        password: String,
    ) -> Result<(User, String), AppError> {
        let is_email_valid = crate::validation::user_validation::validate_email(&email);
        let is_pass_valid = crate::validation::user_validation::validate_password(&password);

        if !is_email_valid || !is_pass_valid {
            println!(
                "Auth Error validating email: {} or password {}",
                email, password
            );
            return Err(AppError::InvalidCredentials);
        } else {
            let user = match users::table
                .filter(users::email.eq(&email))
                .load::<User>(conn)
            {
                Ok(mut users) => match users.pop() {
                    Some(user) => user,
                    _ => {
                        println!("Auth error: No user found with email: {}", &email);
                        return Err(AppError::NotFound);
                    }
                },
                Err(e) => {
                    println!("Authentication error: err getting user from db {:?}", e);
                    return Err(AppError::InternalError);
                }
            };

            AuthUser::verify_password(password, &user)?;
            let token = user.generate_jwt();
            Ok((user, token))
        }
    }

    fn verify_password(pwd: String, user: &User) -> Result<(), AppError> {
        match bcrypt::verify(&pwd, &user.pass) {
            Ok(does_match) => {
                if does_match == true {
                    Ok(())
                } else {
                    println!("Auth error for bcrypt verification for : {}", &user.email);
                    Err(AppError::PasswordHashError)
                }
            }
            Err(e) => {
                println!(
                    "Auth error for bcrypt verification for : {}, error msg: {}",
                    &user.email, e
                );
                Err(AppError::PasswordHashError)
            }
        }
    }
}
