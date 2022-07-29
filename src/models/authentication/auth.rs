use crate::{models::user::{User, self}, schema::users};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use std::io::Error;

#[derive(Queryable, Debug, Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub pass: String,
}

impl AuthUser {
    pub fn authenticate(
        conn: &PgConnection,
        email: String,
        password: String,
    ) -> Result<(User, String), Error> {
        let user = match users::table
            .filter(users::email.eq(email))
            .load::<User>(conn)
        {
            Ok(mut users) => match users.pop() {
                Some(user) => user,
                None => todo!(), //todo
            },
            Err(e) => {
                //println!("Authentication error: err getting user from db {:?}", e);
                todo!()
            }
        };

        let token = user.generate_jwt();

        Ok((user,token))
    }
}
