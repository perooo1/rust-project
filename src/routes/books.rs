use actix_web::{web::{self, Json}, HttpResponse, Responder};

use crate::{db_utils, models::book::{Book, SearchableBook}};

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    let books = Book::get_all_books(&db.get().unwrap());
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::Ok().body("Error getting users"),
    }
}

pub async fn search_by_title(db: web::Data<db_utils::DbPool>, book: Json<SearchableBook>) -> impl Responder {

    let books = SearchableBook::search_book_by_title(&db.get().unwrap(),book.0);
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::Ok().body("Error getting books with provided title"),
    }
}
    
pub async fn search_by_author(db: web::Data<db_utils::DbPool>, book: Json<SearchableBook>) -> impl Responder {

    let books = SearchableBook::search_book_by_author(&db.get().unwrap(),book.0);
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::BadRequest().body("Error getting books with provided author"),
        //Err(_) => HttpResponse::Ok().body("Error getting books with provided author"),
    }
}

pub async fn search_by_publisher(db: web::Data<db_utils::DbPool>, book: Json<SearchableBook>) -> impl Responder {

    let books = SearchableBook::search_book_by_publisher(&db.get().unwrap(),book.0);
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::Ok().body("Error getting books with provided publisher"),
    }
}