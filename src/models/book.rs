use crate::diesel::ExpressionMethods;
use chrono::NaiveDate;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use crate::schema::books::{self};

#[derive(Queryable, PartialEq, Debug, Clone)]
pub struct Book {
    pub book_id: i32,
    pub title: String,
    pub authors: String,
    pub isbn: String,
    pub language_code: String,
    pub num_pages: i32,
    pub publication_date: NaiveDate,
    pub publisher: String,
    pub is_loaned: bool,
}

//Todo get book and update loaned status

impl Book {
    pub fn get_all_books(conn: &PgConnection) -> Result<Vec<Book>, diesel::result::Error> {
        books::table.limit(5).load::<Self>(conn) //remove limit, just for testing
    }

}
