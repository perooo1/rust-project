use crate::custom_errors::app_error::AppError;
use crate::schema::loans::{self};
use crate::validation::{self, user_validation};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::book::Book;

///Struct representing a loan in postgres database
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

///Struct used for creating a new loan in postgres database
#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "loans"]
pub struct NewLoan {
    pub book_id: i32,
    pub user_id: String,
}

impl Loan {
    ///Function for returning all loans from postgres database
    /// # Returns
    /// ## Ok
    /// - Vector of loans in database: Vec<[Loan]>
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    pub fn get_all_loans(conn: &PgConnection) -> Result<Vec<Self>, AppError> {
        loans::table
            .load::<Self>(conn)
            .map_err(|_| AppError::InternalError)
    }
    ///Function for getting a loan with matching id from postgres database
    /// # Returns
    /// ## Ok
    /// - Loan in database: loan: [Loan]
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Not Found error
    pub fn get_loan_by_id(conn: &PgConnection, loan_id: &String) -> Result<Option<Loan>, AppError> {
        match loans::table
            .filter(loans::id.eq(loan_id))
            .load::<Loan>(conn)
        {
            Ok(mut loans) => Ok(loans.pop()),
            Err(_) => Err(AppError::NotFound),
        }
    }

    ///Function for returning a loan
    /// # Returns
    /// ## Ok
    /// - ()
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Not Found error
    /// - Loan Already returned error
    /// - Book Loan status error
    /// - Bad Request
    pub fn return_loan(conn: &PgConnection, id: String) -> Result<(), AppError> {
        match validation::loan_validation::check_if_already_returned(conn, &id) {
            Ok(result) => match result {
                true => {
                    println!("Loan is already returned!");
                    return Err(AppError::LoanReturnedError);
                }
                false => {
                    match diesel::update(loans::table.find(&id))
                        .set(loans::is_returned.eq(true))
                        .execute(conn)
                    {
                        Ok(num_affected) => {
                            if num_affected == 0 {
                                return Err(AppError::NotFound);
                            } else {
                                println!("Update loan status affected {} rows", num_affected);

                                let loan = match Loan::get_loan_by_id(conn, &id) {
                                    Ok(loan) => match loan {
                                        Some(loan) => loan,
                                        None => return Err(AppError::NotFound),
                                    },
                                    Err(_) => return Err(AppError::NotFound),
                                };

                                match Book::update_loan_status(conn, &loan.book_id) {
                                    Ok(success) => match success {
                                        true => Ok(()),
                                        false => Err(AppError::BookLoanStatusError),
                                    },
                                    Err(_) => Err(AppError::BadRequest),
                                }
                            }
                        }
                        Err(_) => {
                            println!("Error updating loan status to true");
                            return Err(AppError::InternalError);
                        }
                    }
                }
            },
            Err(_) => {
                println!("Error returning loan");
                return Err(AppError::NotFound);
            }
        }
    }

    ///Function for checking whether a loan has been returned or not
    /// # Returns
    /// ## Ok
    /// - String with a status message: message: [String]
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Not Found error
    /// - Loan Returned error
    pub fn check_status(conn: &PgConnection, loan_id: String) -> Result<String, AppError> {
        match validation::loan_validation::check_if_already_returned(conn, &loan_id) {
            Ok(result) => match result {
                true => return Err(AppError::LoanReturnedError),
                false => {
                    let loan = match Loan::get_loan_by_id(conn, &loan_id) {
                        Ok(loan) => match loan {
                            Some(loan) => loan,
                            None => return Err(AppError::NotFound),
                        },
                        Err(_) => return Err(AppError::NotFound),
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
                        Ok(String::from("Days until deadline: ")
                            + &time_left.num_days().to_string())
                    }
                }
            },
            Err(_) => {
                println!("Error checking loan status");
                return Err(AppError::NotFound);
            }
        }
    }
    ///Function for returning all loans from a single user
    /// # Returns
    /// ## Ok
    /// - Loan in database: loan: [Loan]
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Not Found error
    pub fn get_loans_for_single_user(
        conn: &PgConnection,
        user_id: String,
    ) -> Result<Vec<Loan>, AppError> {
        match user_validation::user_exists(conn, &user_id) {
            true => {
                match loans::table
                    .filter(loans::user_id.eq(user_id))
                    .load::<Loan>(conn)
                {
                    Ok(loans) => {
                        println!("User currently has no loans!");
                        Ok(loans)
                    }
                    Err(_) => Err(AppError::NotFound),
                }
            }
            false => Err(AppError::NotFound),
        }
    }
}

impl NewLoan {
    ///Function for creating a loan in postgres database
    /// # Returns
    /// ## Ok
    /// - Loan in database: loan: [Loan]
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Not Found error
    /// - Book Loaned error
    /// - Book Loaned status error
    /// - Bad Request
    pub fn create_loan(conn: &PgConnection, b_id: i32, u_id: String) -> Result<Loan, AppError> {
        let user_exists = validation::user_validation::user_exists(conn, &u_id);
        let book_exists = validation::book_validation::book_exists(conn, &b_id);

        if !user_exists || !book_exists {
            println!("User or book with provided id doesn't exist in databse!");
            return Err(AppError::NotFound);
        } else {
            if validation::book_validation::is_book_loaned(conn, &b_id) {
                println!("Book is already loaned!");
                return Err(AppError::BookLoanedError);
            } else {
                match Book::update_loan_status(conn, &b_id) {
                    Ok(is_updated) => match is_updated {
                        true => {
                            let new_loan = Self {
                                book_id: b_id,
                                user_id: u_id,
                            };

                            diesel::insert_into(loans::table)
                                .values(&new_loan)
                                .get_result(conn)
                                .map_err(|_| AppError::InternalError)
                        }
                        false => Err(AppError::BookLoanStatusError),
                    },
                    Err(_) => Err(AppError::BadRequest),
                }
            }
        }
    }
}
