use actix_web::{web::{self, Json}, Responder, HttpResponse, ResponseError};

use crate::{models::book::SearchableBook, db_utils};

/// Search books by author
///
/// # HTTP request
/// * `/books/search/author`
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



#[cfg(test)]
mod tests {
    use actix_web::{
        test,
        web::{self, Data},
        App,
    };

    use crate::{db_utils, models::book::SearchableBook, routes};

    #[actix_web::test]
    async fn test_search_author_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/books/search/author",
            web::get().to(routes::books::search::author::handle),
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
            .uri("/books/search/author")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "GET /books/search/author should return status 200"
        )
    }

    #[actix_web::test]
    async fn test_search_author_invalid() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/books/search/author",
            web::get().to(routes::books::search::author::handle),
        ))
        .await;

        let dummy_search = SearchableBook {
            title: "harry".to_string(),
            authors: "".to_string(),
            publisher: "books".to_string(),
        };

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_search)
            .uri("/books/search/author")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "GET /books/search/author should return status 400"
        )
    }
}
