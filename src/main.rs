#[macro_use]
extern crate diesel;

mod application;
mod db_utils;
mod models;
mod routes;
mod schema;

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
