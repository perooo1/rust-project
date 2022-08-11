use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::book::Book};
/// Get books from db
///
/// # HTTP request
/// * `/books`
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Response is in [Json] format
/// ```
/// [
/// {
///     "book_id": 12
///     "title": "The Ultimate Hitchhiker's Guide (Hitchhiker's Guide to the Galaxy  #1-5)"
///     "authors": "Douglas Adams"
///     "isbn": "0517149257"
///     "language_code": "eng"
///     "num_pages": 815
///     "publication_date": "1996-01-17"
///     "publisher": "Wings Books"
///     "is_loaned" : false
/// }
/// ]
/// ```
/// Error code: 500
pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    let books = Book::get_all_books(&db.get().unwrap());
    match books {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(e) => e.error_response(),
    }
}


#[cfg(test)]
mod tests {
    use actix_web::{
        test,
        web::{self, Data},
        App,
    };

    use crate::{db_utils, routes};

    #[actix_web::test]
    async fn test_get_books_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/books", web::get().to(routes::books::index::handle)),
        )
        .await;

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .uri("/books")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "GET /books should return status 200")
    }
}

