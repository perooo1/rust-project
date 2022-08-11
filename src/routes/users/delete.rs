use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::user::User};

/// Delete user
///
/// # HTTP request
/// * `/users/{id}/delete`
///
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Returns a number of rows affected in postgres database
///
/// Error code: 404, 500
pub async fn handle(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let user_id = path.into_inner();
    match User::delete_user(&db.get().unwrap(), user_id) {
        Ok(cnt) => HttpResponse::Ok().json(cnt),
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
    async fn test_delete_user_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/users/{id}/delete",
            web::put().to(routes::users::delete::handle),
        ))
        .await;

        let req = test::TestRequest::put()
            .app_data(Data::new(db_pool.clone()))
            .uri("/users/977214ab-7889-4bb5-9455-96354579e4a5/delete")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "PUT /users/id/delete should return status 200"
        )
    }

    #[actix_web::test]
    async fn test_delete_user_invalid() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(App::new().app_data(Data::new(db_pool.clone())).route(
            "/users/{id}/delete",
            web::put().to(routes::users::delete::handle),
        ))
        .await;

        let req = test::TestRequest::put()
            .app_data(Data::new(db_pool.clone()))
            .uri("/users/977214ab-7889-4bb5-NEMA-96354579e4a5/delete")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            404,
            "PUT /users/id/delete should return status 404"
        )
    }
}
