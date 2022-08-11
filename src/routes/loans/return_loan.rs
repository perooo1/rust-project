use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::loan::Loan};

/// Return a loan
///
/// # HTTP request
/// * `/loans/{id}/return`
///
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Returns a number of rows affected in postgres database
///
/// Error code: 400, 404, 500
pub async fn handle(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let loan_id = path.into_inner();

    match Loan::return_loan(&db.get().unwrap(), loan_id) {
        Ok(()) => HttpResponse::Ok().body("Successfully returned a loan!"),
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
    async fn test_return_loan_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/loans/{id}/return",
            web::put().to(routes::loans::return_loan::handle),
        ))
        .await;

        let req = test::TestRequest::put()
            .app_data(Data::new(db_pool.clone()))
            .uri("/loans/c778213d-2428-4974-b2cb-80ae33f75bb8/return")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "PUT /loans/id/return should return status 200"
        )
    }

    #[actix_web::test]
    async fn test_return_loan_invalid() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/loans/{id}/return",
            web::put().to(routes::loans::return_loan::handle),
        ))
        .await;

        let req = test::TestRequest::put()
            .app_data(Data::new(db_pool.clone()))
            .uri("/loans/c778213d-NEMA-4974-b2cb-80ae33f75bb8/return")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            404,
            "PUT /loans/id/return should return status 404"
        )
    }
}
