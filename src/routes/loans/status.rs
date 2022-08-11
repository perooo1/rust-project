use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::loan::Loan};

/// Check loan status
///
/// # HTTP request
/// * `/loans/{id}/status`
/// Request must be in [Json] format
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Response is a [String] status message
/// Error code: 400, 404, 500
pub async fn handle(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let loan_id = path.into_inner();

    match Loan::check_status(&db.get().unwrap(), loan_id) {
        Ok(status_msg) => HttpResponse::Ok().body(status_msg),
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
    async fn test_loan_status_already_returned() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/loans/{id}/status",
            web::get().to(routes::loans::status::handle),
        ))
        .await;

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .uri("/loans/c778213d-2428-4974-b2cb-80ae33f75bb8/status")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "GET /loans/id/status should return status 400"
        )
    }

    #[actix_web::test]
    async fn test_loan_status_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/loans/{id}/status",
            web::get().to(routes::loans::status::handle),
        ))
        .await;

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .uri("/loans/75c2621d-6fad-4cd7-a564-53b2aefdff01/status")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "GET /loans/id/status should return status 400"
        )
    }

    #[actix_web::test]
    async fn test_loan_status_invalid() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/loans/{id}/status",
            web::get().to(routes::loans::status::handle),
        ))
        .await;

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .uri("/loans/c778213d-NEMA-4974-b2cb-80ae33f75bb8/status")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            404,
            "GET /loans/id/status should return status 404"
        )
    }
}
