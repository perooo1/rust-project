#[macro_use]
extern crate diesel;

mod application;
mod db_utils;
mod errors;
mod models;
mod routes;
mod schema;
mod validation;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(db_utils::establish_pool_connection().clone()))
            .service(web::resource("/register").route(web::post().to(routes::register::handle)))
            .service(web::resource("/users").route(web::get().to(routes::users::handle)))
            .service(
                web::resource("/users/{id}/delete").route(web::delete().to(routes::users::del_usr)),
            )
            .service(web::resource("/login").route(web::post().to(routes::login::handle)))
            .service(web::resource("/books").route(web::get().to(routes::books::handle)))
            .service(web::resource("/books/search/title").route(web::get().to(routes::books::search_by_title)))
            .service(web::resource("/books/search/author").route(web::get().to(routes::books::search_by_author)))
            .service(web::resource("/books/search/publisher").route(web::get().to(routes::books::search_by_publisher)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
