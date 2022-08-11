use actix_web::{
    web::{self, Json},
    HttpResponse, Responder, ResponseError,
};

use crate::{db_utils, models::book::SearchableBook};

/// Search books by publisher
///
/// # HTTP request
/// * `/books/search/publisher`
/// * Request must be in [Json] format
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
/// Error code: 400, 500
pub async fn handle(db: web::Data<db_utils::DbPool>, book: Json<SearchableBook>) -> impl Responder {
    let books = SearchableBook::search_book_by_publisher(&db.get().unwrap(), book.0);
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

    use crate::{db_utils, models::book::SearchableBook, routes};

    #[actix_web::test]
    async fn test_search_publisher_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/books/search/publisher",
            web::get().to(routes::books::search::publisher::handle),
        ))
        .await;

        let dummy_search = SearchableBook {
            title: "Harry".to_string(),
            authors: "rowling".to_string(),
            publisher: "books".to_string(),
        };

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_search)
            .uri("/books/search/publisher")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "GET /books/search/publisher should return status 200"
        )
    }

    #[actix_web::test]
    async fn test_search_publisher_invalid() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/books/search/publisher",
            web::get().to(routes::books::search::publisher::handle),
        ))
        .await;

        let dummy_search = SearchableBook {
            title: "harry".to_string(),
            authors: "rowling".to_string(),
            publisher: "".to_string(),
        };

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_search)
            .uri("/books/search/publisher")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "GET /books/search/publisher should return status 400"
        )
    }
}
