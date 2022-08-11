use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::custom_errors::app_error::AppError;
use crate::schema::books;
use crate::validation;

///Struct representing a book in database
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

///Struct used for searching a book in database
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct SearchableBook {
    pub title: String,
    pub authors: String,
    pub publisher: String,
}

impl Book {
    ///Function for returning all books from postgres database
    /// # Returns
    /// ## Ok
    /// - Vector of available books in database: Vec<[Book]>
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    pub fn get_all_books(conn: &PgConnection) -> Result<Vec<Book>, AppError> {
        books::table
            .limit(5)
            .load::<Self>(conn)
            .map_err(|_| AppError::InternalError) //remove limit, just for testing
    }

    ///Function for returning a book with a matching id
    pub fn get_book_by_id(conn: &PgConnection, id: &i32) -> Result<Book, AppError> {
        match books::table.filter(books::id.eq(id)).load::<Book>(conn) {
            Ok(mut books) => Ok(match books.pop() {
                Some(book) => book,
                None => return Err(AppError::NotFound),
            }),
            Err(_) => Err(AppError::InternalError),
        }
    }

    ///Function for updating a loan status
    /// # Returns
    /// ## Ok
    /// - is_updated: [bool]
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Not Found error
    pub fn update_loan_status(conn: &PgConnection, id: &i32) -> Result<bool, AppError> {
        let book = match Book::get_book_by_id(conn, id) {
            Ok(book) => book,
            Err(e) => {
                println!("book finding by id error {:?}", e.to_string());
                return Err(AppError::NotFound);
            }
        };

        if book.is_loaned == true {
            match diesel::update(books::table.find(id))
                .set(books::is_loaned.eq(false))
                .execute(conn)
            {
                Ok(num_affected) => {
                    println!("Update loan status affected {} rows", num_affected);

                    if num_affected == 0 {
                        Err(AppError::NotFound)
                    } else {
                        Ok(true)
                    }
                }
                Err(_) => {
                    println!("Error updating loan status to false");
                    Err(AppError::InternalError)
                }
            }
        } else {
            match diesel::update(books::table.find(id))
                .set(books::is_loaned.eq(true))
                .execute(conn)
            {
                Ok(num_affected) => {
                    println!("Update loan status affected {} rows", num_affected);

                    if num_affected == 0 {
                        Err(AppError::NotFound)
                    } else {
                        Ok(true)
                    }
                }
                Err(_) => {
                    println!("Error updating loan status to false");
                    Err(AppError::InternalError)
                }
            }
        }
    }
}

impl SearchableBook {
    ///Function for returning all books with a matching title query from postgres database
    /// # Returns
    /// ## Ok
    /// - Vector of available books in database: Vec<[Book]>
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Bad Request
    pub fn search_book_by_title(
        conn: &PgConnection,
        book: SearchableBook,
    ) -> Result<Vec<Book>, AppError> {
        let is_title_empty = validation::book_validation::is_book_title_empty(&book.title);

        if is_title_empty {
            return Err(AppError::BadRequest);
        } else {
            match diesel::QueryDsl::filter(
                books::table,
                books::title.ilike(("%".to_owned() + &book.title + "%").trim()),
            )
            .load::<Book>(conn)
            {
                Ok(books) => Ok(books),
                Err(_) => Err(AppError::InternalError),
            }
        }
    }
    ///Function for returning all books with a matching author query from postgres database
    /// # Returns
    /// ## Ok
    /// - Vector of available books in database: Vec<[Book]>
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Bad Request
    pub fn search_book_by_author(
        conn: &PgConnection,
        book: SearchableBook,
    ) -> Result<Vec<Book>, AppError> {
        let is_author_empty = validation::book_validation::is_book_author_empty(&book.authors);

        if is_author_empty {
            return Err(AppError::BadRequest);
        } else {
            match diesel::QueryDsl::filter(
                books::table,
                books::authors.ilike(("%".to_owned() + &book.authors + "%").trim()),
            )
            .load::<Book>(conn)
            {
                Ok(books) => Ok(books),
                Err(_) => Err(AppError::InternalError),
            }
        }
    }
    ///Function for returning all books with a matching publisher query from postgres database
    /// # Returns
    /// ## Ok
    /// - Vector of available books in database: Vec<[Book]>
    /// ## Error
    /// - error: [AppError]
    /// - Internal server error
    /// - Bad Request
    pub fn search_book_by_publisher(
        conn: &PgConnection,
        book: SearchableBook,
    ) -> Result<Vec<Book>, AppError> {
        let is_publisher_empty =
            validation::book_validation::is_book_publisher_empty(&book.publisher);

        if is_publisher_empty {
            return Err(AppError::BadRequest);
        } else {
            match diesel::QueryDsl::filter(
                books::table,
                books::publisher.ilike(("%".to_owned() + &book.publisher + "%").trim()),
            )
            .load::<Book>(conn)
            {
                Ok(books) => Ok(books),
                Err(_) => Err(AppError::InternalError),
            }
        }
    }
}
