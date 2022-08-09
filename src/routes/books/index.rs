use actix_web::{
    web,
    HttpResponse, Responder, ResponseError,
};

use crate::{
    db_utils,
    models::book::Book,
};

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    let books = Book::get_all_books(&db.get().unwrap());
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => e.error_response(),
    }
}
