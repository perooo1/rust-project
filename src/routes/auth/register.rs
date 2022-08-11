use crate::db_utils;
use crate::models::user::NewUser;
use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse, Responder, ResponseError,
};

/// Register user
///
/// # HTTP request
/// Request must be in [Json] format
/// ## Body
/// * username: [String]
/// * password: [String] - minimum 8 characters long, one uppercase, one number and no spaces
///
/// # HTTP response
/// ##Header
/// * Success code: 200
/// *
/// ## Body
/// * Response is in [Json] format
/// ```
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
/// ```
/// Error code: 400, 500
pub async fn handle(db: web::Data<db_utils::DbPool>, user: Json<NewUser>) -> impl Responder {
    match NewUser::create_user(
        &db.get().unwrap(),
        user.first_name.to_string(),
        user.last_name.to_string(),
        user.email.to_string(),
        user.pass.to_string(),
        user.is_admin,
    ) {
        Ok(created_user) => HttpResponse::Ok().json(created_user),
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

    use crate::{db_utils, models::user::NewUser, routes};

    #[actix_web::test]
    async fn test_register_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/register", web::post().to(routes::auth::register::handle)),
        )
        .await;

        let dummy_user = NewUser {
            first_name: "dummy".to_string(),
            last_name: "dummy_lastname".to_string(),
            email: "dummy@email.com".to_string(),
            pass: "DummyPass123".to_string(),
            is_admin: false,
        };

        let req = test::TestRequest::post()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_user)
            .uri("/register")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "POST /register should return status 200"
        )
    }

    #[actix_web::test]
    async fn test_register_invalid_email() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/register", web::post().to(routes::auth::register::handle)),
        )
        .await;

        let dummy_user = NewUser {
            first_name: "dummy".to_string(),
            last_name: "dummy_lastname".to_string(),
            email: "dummy.com".to_string(),
            pass: "DummyPass123".to_string(),
            is_admin: false,
        };

        let req = test::TestRequest::post()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_user)
            .uri("/register")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "POST /register should return status 400"
        )
    }

    #[actix_web::test]
    async fn test_register_invalid_password() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/register", web::post().to(routes::auth::register::handle)),
        )
        .await;

        let dummy_user = NewUser {
            first_name: "dummy".to_string(),
            last_name: "dummy_lastname".to_string(),
            email: "dummy2@email.com".to_string(),
            pass: "pwrd".to_string(),
            is_admin: false,
        };

        let req = test::TestRequest::post()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_user)
            .uri("/register")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "POST /register should return status 400"
        )
    }
}
