use diesel::PgConnection;

use crate::models::loan::Loan;

pub fn check_if_already_returned(conn: &PgConnection, id: &String) -> bool {
    let loan = match Loan::get_loan_by_id(conn, &id.to_string()) {
        Ok(loan) => loan.unwrap(),
        Err(_) => {
            println!("loan finding by id error");
            return false;
        }
    };

    return loan.is_returned;
}
