use actix_web::{web::{self, Json}, Responder, HttpResponse, ResponseError};

use crate::{models::book::SearchableBook, db_utils};

pub async fn handle(
    db: web::Data<db_utils::DbPool>,
    book: Json<SearchableBook>,
) -> impl Responder {
    let books = SearchableBook::search_book_by_author(&db.get().unwrap(), book.0);
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => e.error_response(),
    }
}