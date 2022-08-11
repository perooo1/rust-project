use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::loan::Loan};

/// Get user's loans
///
/// # HTTP request
/// * `/users/{id}/loans`
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Response is in [Json] format
/// ```
/// [
/// {
///     "id": "f7169845-4de5-470e-bb76-7117d4620d8c"
///     "first_name" : "ImeTest"
///     "last_name" : "PrezimeTest"
///     "email" : "ime@prezime.test"
///     "pass" : "$2b$12$N69J59jMGZ9iNafgThD1kunT3XyXr2SpptF4GAZ0cWtuXbGu9Ea0u"
///     "is_admin" : false
///     "created_at" : "2022-07-27T07:05:09.877660"
///     "updated_at" : "2022-07-27T07:05:09.877660"
///     "is_deleted" : false
/// }
/// ]
/// ```
/// Error code: 400, 404, 500
pub async fn handle(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let user_id = path.into_inner();
    match Loan::get_loans_for_single_user(&db.get().unwrap(), user_id) {
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
    async fn test_get_user_loans_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/users/{id}/loans",
            web::get().to(routes::users::loans::handle),
        ))
        .await;

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .uri("/users/696b5858-9136-4f3d-8674-116a9ee4ea33/loans")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "GET /users/id/loans should return status 200"
        )
    }

    #[actix_web::test]
    async fn test_get_user_loans_invalid_user() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/users/{id}/loans",
            web::get().to(routes::users::loans::handle),
        ))
        .await;

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .uri("/users/696b5858-9136-4f3d-8674-116a9ee4ea36/loans")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "GET /users/id/loans should return status 400"
        )
    }
}
