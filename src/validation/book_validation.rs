use diesel::PgConnection;

use crate::models::book::Book;

///Check if book title is empty
pub fn is_book_title_empty(title: &String) -> bool {
    title.is_empty()
}

///Check if book author is empty
pub fn is_book_author_empty(author: &String) -> bool {
    author.is_empty()
}

///Check if book publisher is empty
pub fn is_book_publisher_empty(publisher: &String) -> bool {
    publisher.is_empty()
}

///Check if book is in postgres databse
pub fn book_exists(conn: &PgConnection, id: &i32) -> bool {
    match Book::get_book_by_id(conn, id) {
        Ok(_) => true,
        Err(_) => false,
    }
}

///Check if book is already loaned
pub fn is_book_loaned(conn: &PgConnection, id: &i32) -> bool {
    let book = match Book::get_book_by_id(conn, id) {
        Ok(book) => book,
        Err(e) => {
            println!("book finding by id error");
            return false;
        }
    };

    return book.is_loaned;
}
