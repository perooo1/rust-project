use crate::schema::loans::{self};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Queryable, PartialEq, Debug, Clone)]

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

#[derive(Queryable, Insertable, Debug)]
#[table_name = "loans"]
pub struct NewLoan {
    pub book_id: i32,
    pub user_id: String,
    pub loan_date: NaiveDate,
    pub return_deadline: NaiveDate,                 //postaviti na 14 dana nakon loan_date
    pub is_returned: bool,
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
        connection: &PgConnection,
        b_id: i32,
        u_id: String,
        l_date: NaiveDate,
        deadline: NaiveDate,
        returned: bool,
    ) -> Result<Loan, diesel::result::Error> {
        let new_loan = Self {
            book_id: b_id,
            user_id: u_id,
            loan_date: l_date,
            return_deadline: deadline,
            is_returned: returned,
        };

        diesel::insert_into(loans::table)
            .values(&new_loan)
            .get_result(connection)
            //.expect("Error creating a new loan")
    }
}
