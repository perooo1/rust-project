use diesel::PgConnection;

use crate::custom_errors::app_error::AppError;
use crate::models::loan::Loan;

pub fn check_if_already_returned(conn: &PgConnection, id: &String) -> Result<bool, AppError> {
    let loan: Loan = match Loan::get_loan_by_id(conn, &id.to_string()) {
        Ok(loan) => match loan {
            Some(loan) => loan,
            None => return Err(AppError::NotFound),
        },
        Err(_) => {
            println!("loan finding by id error");
            return Ok(false);
        }
    };

    return Ok(loan.is_returned);
}
