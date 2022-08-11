#[macro_use]
extern crate diesel;

/**
 API for library management system. Supports basic user functions - register and login(using stateless [jwt]).
 Supports searching for a book either by title, author or publihser, loaning a book, returning a book, checking loan status
 */

mod application;
mod custom_errors;
mod db_utils;
mod models;
mod routes;
mod schema;
mod validation;

use actix_web;

///Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    application::setup_web_server().await
}
