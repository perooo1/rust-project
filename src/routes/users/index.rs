use crate::{db_utils, models::user::User};
use actix_web::ResponseError;
use actix_web::{web, HttpResponse, Responder};

/// Get users from db
///
/// # HTTP request
/// * `/users`
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
/// Error code: 500
pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    let users = User::get_all_users(&db.get().unwrap());
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
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
    async fn test_get_users() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/users", web::get().to(routes::users::index::handle)),
        )
        .await;

        let req = test::TestRequest::get()
            .app_data(Data::new(db_pool.clone()))
            .uri("/users")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "GET /users should return status 200")
    }
}
