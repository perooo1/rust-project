use actix_web::{web, web::Json, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::authentication::auth::AuthUser};

/// Login user
///
/// # HTTP request
/// Request must be in [Json] format
/// ## Header
/// * jwt: [String] - JWT autorization token
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
/// Error code: 400, 404, 500
pub async fn handle(db: web::Data<db_utils::DbPool>, user: Json<AuthUser>) -> impl Responder {
    match AuthUser::authenticate(
        &db.get().unwrap(),
        user.email.to_string(),
        user.pass.to_string(),
    ) {
        Ok((auth_user, token)) => HttpResponse::Ok()
            .append_header(("jwt", token))
            .json(auth_user),
        Err(e) => e.error_response(),
    }
}

#[cfg(test)]
mod test {
    use actix_web::{
        test,
        web::{self, Data},
        App,
    };

    use crate::{db_utils, models::authentication::auth::AuthUser, routes};

    #[actix_web::test]
    async fn test_login_ok() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/login", web::post().to(routes::auth::login::handle)),
        )
        .await;

        let dummy_user = AuthUser {
            email: "antisa@gmail.com".to_string(),
            pass: "Ante123456".to_string(),
        };

        let req = test::TestRequest::post()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_user)
            .uri("/login")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            200,
            "POST /register should return status 200"
        )
    }

    #[actix_web::test]
    async fn test_login_invalid_email() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/login", web::post().to(routes::auth::login::handle)),
        )
        .await;

        let dummy_user = AuthUser {
            email: "antisa.com".to_string(),
            pass: "Ante123456".to_string(),
        };

        let req = test::TestRequest::post()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_user)
            .uri("/login")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "POST /register should return status 400"
        )
    }

    #[actix_web::test]
    async fn test_login_invalid_pwd() {
        let db_pool = db_utils::establish_pool_connection();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(db_pool.clone()))
                .route("/login", web::post().to(routes::auth::login::handle)),
        )
        .await;

        let dummy_user = AuthUser {
            email: "antisa@gmail.com".to_string(),
            pass: "ante123".to_string(),
        };

        let req = test::TestRequest::post()
            .app_data(Data::new(db_pool.clone()))
            .set_json(dummy_user)
            .uri("/login")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            400,
            "POST /register should return status 400"
        )
    }
}
