//setup web server && all the routes
//pub async fun setup_web_server


/* 
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};

use crate::db_utils;
use crate::routes;

pub async fn setup_web_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(db_utils::initialize().clone()))            //potencijalni .clone()
            .service(web::scope("/").configure(setup_routes))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn setup_routes(cfg: &mut web::ServiceConfig) {
    //POST /register
    cfg.service(web::resource("/register").route(web::post().to(routes::register::handle)));
    //POST /login
    cfg.service(web::resource("/login").route(web::post().to(routes::login::handle)));
    // GET /users
    cfg.service(web::resource("/users").route(web::get().to(routes::users::handle)));
    // DELETE /users/{id}/delete
    cfg.service(
        web::resource("/users/{id}/delete").route(web::delete().to(routes::users::del_usr)),
    );
}
*/