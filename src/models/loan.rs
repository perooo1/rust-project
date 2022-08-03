use crate::schema::loans::{self};
use crate::validation;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::book::Book;

#[derive(Queryable, PartialEq, Debug, Clone, Serialize, Deserialize)]

pub struct Loan {
    pub id: String,
    pub book_id: i32,
    pub user_id: String,
    pub loan_date: NaiveDate,
    pub return_deadline: NaiveDate,
    pub is_returned: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "loans"]
pub struct NewLoan {
    pub book_id: i32,
    pub user_id: String,
}

impl Loan {
    pub fn get_loan_by_id(
        conn: &PgConnection,
        loan_id: String,
    ) -> Result<Option<Loan>, diesel::result::Error> {
        match loans::table
            .filter(loans::id.eq(loan_id))
            .load::<Loan>(conn)
        {
            Ok(mut loans) => Ok(loans.pop()),
            Err(e) => Err(e),
        }
    }
}

impl NewLoan {
    pub fn create_loan(
        conn: &PgConnection,
        b_id: i32,
        u_id: String,
    ) -> Result<Loan, diesel::result::Error> {
        //check if user id is correct
        //check if book id is correct, if correct, check if book is already loaned, else add book id to loan table and change loaned status in book
        let user_exists = validation::user_validation::user_exists(conn, &u_id);
        let book_exists = validation::book_validation::book_exists(conn, &b_id);

        if !user_exists || !book_exists {
            println!("User or book with provided id doesn't exist in databse!");
            return Err(diesel::result::Error::NotFound); //ne ovo koristit za error!!
        } else {
            if validation::book_validation::is_book_already_loaned(conn, &b_id) {
                println!("Book is already loaned!");
                return Err(diesel::result::Error::__Nonexhaustive); //ne ovo koristit za error!!
            } else {
                Book::update_loan_status(conn, &b_id);

                let new_loan = Self {
                    book_id: b_id,
                    user_id: u_id,
                };

                diesel::insert_into(loans::table)
                    .values(&new_loan)
                    .get_result(conn)
            }
        }
    }
}
