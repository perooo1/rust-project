#[macro_use]
extern crate diesel;

mod application;
mod custom_errors;
mod db_utils;
mod models;
mod routes;
mod schema;
mod validation;

use actix_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    application::setup_web_server().await
}
