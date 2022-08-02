use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::books::{self};
use crate::validation;

#[derive(Queryable, PartialEq, Debug, Clone, Serialize)]
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
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct SearchableBook {
    pub title: String,
    pub authors: String,
    pub publisher: String,
}

//Todo get book and update loaned status

impl Book {
    pub fn get_all_books(conn: &PgConnection) -> Result<Vec<Book>, diesel::result::Error> {
        books::table.limit(5).load::<Self>(conn) //remove limit, just for testing / should probably be paginated
    }

    //todo update loaned status method
}

impl SearchableBook {
    pub fn search_book_by_title(
        conn: &PgConnection,
        book: SearchableBook,
    ) -> Result<Vec<Book>, diesel::result::Error> {
        match diesel::QueryDsl::filter(
            books::table,
            books::title.ilike(("%".to_owned() + &book.title + "%").trim()),
        )
        .load::<Book>(conn)
        {
            Ok(books) => Ok(books),
            Err(e) => Err(e),
        }
    }

    pub fn search_book_by_author(
        conn: &PgConnection,
        book: SearchableBook,
    ) -> Result<Vec<Book>, diesel::result::Error> {
        let is_author_empty = validation::is_book_author_empty(&book.authors);

        if is_author_empty {
            return Err(diesel::result::Error::__Nonexhaustive);
        } else {
            match diesel::QueryDsl::filter(
                books::table,
                books::authors.ilike(("%".to_owned() + &book.authors + "%").trim()),
            )
            .load::<Book>(conn)
            {
                Ok(books) => Ok(books),
                Err(e) => Err(e),
            }
        }
    }

    pub fn search_book_by_publisher(
        conn: &PgConnection,
        book: SearchableBook,
    ) -> Result<Vec<Book>, diesel::result::Error> {
        match diesel::QueryDsl::filter(
            books::table,
            books::publisher.ilike(("%".to_owned() + &book.publisher + "%").trim()),
        )
        .load::<Book>(conn)
        {
            Ok(books) => Ok(books),
            Err(e) => Err(e),
        }
    }
}
