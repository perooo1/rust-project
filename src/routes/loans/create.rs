use actix_web::{
    web::{self, Json},
    HttpResponse, Responder, ResponseError,
};

use crate::{db_utils, models::loan::NewLoan};

/// Create a new loan
///
/// # HTTP request
/// * `/loans/create`
/// Request must be in [Json] format
/// ## Body
/// * book_id: [i32]
/// * user_id: [String]
///
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Response is in [Json] format
/// ```
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
/// ```
/// Error code: 400, 404, 500
pub async fn handle(db: web::Data<db_utils::DbPool>, loan: Json<NewLoan>) -> impl Responder {
    match NewLoan::create_loan(&db.get().unwrap(), loan.book_id, loan.user_id.to_string()) {
        Ok(created_loan) => HttpResponse::Ok().json(created_loan),
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

    use crate::{
        db_utils,
        models::{loan::NewLoan, user::NewUser},
        routes,
    };

    #[actix_web::test]
    async fn test_create_loan_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/loans/create",
            web::post().to(routes::loans::create::handle),
        ))
        .await;

        let dummy_loan = NewLoan {
            book_id: 69,
            user_id: "a0b1782b-6998-46a9-9e9d-cc6e3f418090".to_string(),
        };

        let req = test::TestRequest::post()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_loan)
            .uri("/loans/create")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "POST /loans/create should return status 200"
        )
    }

    #[actix_web::test]
    async fn test_create_loan_invalid_user() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/loans/create",
            web::post().to(routes::loans::create::handle),
        ))
        .await;

        let dummy_loan = NewLoan {
            book_id: 69,
            user_id: "a0b1782b-6998-NEMA-9e9d-cc6e3f418090".to_string(),
        };

        let req = test::TestRequest::post()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_loan)
            .uri("/loans/create")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "POST /loans/create should return status 400"
        )
    }

}
