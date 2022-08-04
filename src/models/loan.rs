use crate::schema::loans::{self};
use crate::validation;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use r2d2::Error;
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

    pub fn return_loan(conn: &PgConnection, id: String) -> Result<(), diesel::result::Error> {
        if validation::loan_validation::check_if_already_returned(conn, &id) {
            println!("Loan is already returned!");
            return Err(diesel::result::Error::__Nonexhaustive); //ne ovo koristit za error!!
        } else {
            match diesel::update(loans::table.find(&id))
                .set(loans::is_returned.eq(true))
                .execute(conn)
            {
                Ok(num_affected) => {
                    println!("Update loan status affected {} rows", num_affected);

                    let loan = match Loan::get_loan_by_id(conn, id) {
                        Ok(loan) => match loan {
                            Some(loan) => loan,
                            None => return Err(diesel::result::Error::NotFound),
                        },
                        Err(_) => return Err(diesel::result::Error::NotFound),
                    };

                    Ok(Book::update_loan_status(conn, &loan.book_id))
                }
                Err(_) => {
                    println!("Error updating loan status to true");
                    return Err(diesel::result::Error::__Nonexhaustive);
                }
            }
        }
    }

    pub fn check_deadline_status(
        conn: &PgConnection,
        loan_id: String,
    ) -> Result<String, diesel::result::Error> {
        //todo update da gleda opcenito status
        let loan = match Loan::get_loan_by_id(conn, loan_id) {
            Ok(loan) => match loan {
                Some(loan) => loan,
                None => return Err(diesel::result::Error::NotFound),
            },
            Err(_) => return Err(diesel::result::Error::NotFound),
        };

        let current_date = Utc::now().naive_utc().date();

        if current_date > loan.return_deadline {
            let overtime = current_date - loan.return_deadline;
            Ok(
                String::from("You are late returning a book. Days overtime: ")
                    + &overtime.num_days().to_string(),
            )
        } else {
            let time_left = loan.return_deadline - current_date;
            Ok(String::from("Time until deadline: ") + &time_left.num_days().to_string())
        }
    }

    pub fn get_loans_for_single_user(
        conn: &PgConnection,
        user_id: String,
    ) -> Result<Vec<Loan>, diesel::result::Error> {
        match loans::table
            .filter(loans::user_id.eq(user_id))
            .load::<Loan>(conn)
        {
            Ok(loans) => Ok(loans),
            Err(_) => Err(diesel::result::Error::NotFound),
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
            if validation::book_validation::is_book_loaned(conn, &b_id) {
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
