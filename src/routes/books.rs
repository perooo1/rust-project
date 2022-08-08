use actix_web::{
    web::{self, Json},
    HttpResponse, Responder, ResponseError,
};

use crate::{
    db_utils,
    models::book::{Book, SearchableBook},
};

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    let books = Book::get_all_books(&db.get().unwrap());
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => e.error_response(),
    }
}

pub async fn search_by_title(
    db: web::Data<db_utils::DbPool>,
    book: Json<SearchableBook>,
) -> impl Responder {
    let books = SearchableBook::search_book_by_title(&db.get().unwrap(), book.0);
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => e.error_response(),
    }
}

pub async fn search_by_author(
    db: web::Data<db_utils::DbPool>,
    book: Json<SearchableBook>,
) -> impl Responder {
    let books = SearchableBook::search_book_by_author(&db.get().unwrap(), book.0);
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => e.error_response(),
    }
}

pub async fn search_by_publisher(
    db: web::Data<db_utils::DbPool>,
    book: Json<SearchableBook>,
) -> impl Responder {
    let books = SearchableBook::search_book_by_publisher(&db.get().unwrap(), book.0);
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => e.error_response(),
    }
}
