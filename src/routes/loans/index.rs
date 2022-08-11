use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::loan::Loan};
/// Get loans from db
///
/// # HTTP request
/// * `/loans`
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Response is in [Json] format
/// ```
/// [
/// {
///     "id": "6c45d5c6-52b7-48b7-80ba-d80ac229a773"
///     "book_id": 8
///     "user_id": "dd728f49-5372-4e69-b60a-95da7571e510"
///     "loan_date": "2022-08-04",
///     "return_deadline": "2022-08-18",
///     "is_returned": true
///     "created_at": "2022-08-04T11:12:17.777218"
///     "updated_at": "2022-08-04T11:13:38.309337"
/// }
/// ]
/// ```
/// Error code: 500
pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    match Loan::get_all_loans(&db.get().unwrap()) {
        Ok(loans) => HttpResponse::Ok().json(loans),
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
    async fn test_get_loans() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/loans", web::get().to(routes::loans::index::handle)),
        )
        .await;

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .uri("/loans")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "GET /loans should return status 200")
    }
}
