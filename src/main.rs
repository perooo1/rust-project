
#[macro_use]
extern crate diesel;

mod db_utils;
mod application;
mod models;
mod schema;
mod routes;

use actix_web::{web::{self, Data}, App, HttpServer};
use models::{user,loan,book};
//use db_utils::establish_connection;



#[actix_web::main]
async fn main() -> std::io::Result<()> {

/* 
    println!("Hello, world!");
    let connection = establish_connection(); //just testing diesel models
    let books = book::Book::get_all_books(&connection).unwrap();
    println!("Books: {:?}", books);
*/

    HttpServer::new( ||{
        App::new()
        .app_data(Data::new(db_utils::establish_pool_connection().clone()))
        .service(web::resource("/register").route(web::post().to(routes::register::handle)))
        .service(web::resource("/users").route(web::get().to(routes::users::handle)))
        //.route("/register", web::post().to(routes::register::handle(conn, user)))

    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await


}


