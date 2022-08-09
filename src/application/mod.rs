use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

use crate::{db_utils, routes};

pub async fn setup_web_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(db_utils::establish_pool_connection().clone()))
            //POST /register
            .service(
                web::resource("/register").route(web::post().to(routes::auth::register::handle)),
            )
            //POST /login
            .service(web::resource("/login").route(web::post().to(routes::auth::login::handle)))
            //GET /users
            .service(web::resource("/users").route(web::get().to(routes::users::index::handle)))
            //GET /users/{id}/loans
            .service(
                web::resource("/users/{id}/loans")
                    .route(web::get().to(routes::users::loans::handle)),
            )
            //DELETE /users/{id}/delete
            .service(
                web::resource("/users/{id}/delete")
                    .route(web::delete().to(routes::users::delete::handle)),
            )
            //GET /books
            .service(web::resource("/books").route(web::get().to(routes::books::index::handle)))
            //GET /books/search/title
            .service(
                web::resource("/books/search/title")
                    .route(web::get().to(routes::books::search::title::handle)),
            )
            //GET /books/search/author
            .service(
                web::resource("/books/search/author")
                    .route(web::get().to(routes::books::search::author::handle)),
            )
            //GET /books/search/publisher
            .service(
                web::resource("/books/search/publisher")
                    .route(web::get().to(routes::books::search::publisher::handle)),
            )
            //GET /loans
            .service(web::resource("/loans").route(web::get().to(routes::loans::index::handle)))
            //GET /loans/{id}/status
            .service(
                web::resource("/loans/{id}/status")
                    .route(web::get().to(routes::loans::status::handle)),
            )
            //PUT /loans/create
            .service(
                web::resource("/loans/create").route(web::post().to(routes::loans::create::handle)),
            )
            //PUT /loans/{id}/return
            .service(
                web::resource("/loans/{id}/return")
                    .route(web::put().to(routes::loans::return_loan::handle)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
