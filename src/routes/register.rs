use crate::db_utils;
use crate::models::user::NewUser;
use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};


/*
pub async fn handle(conn: &PgConnection, user: Json<NewUser>) -> impl Responder {

    //let usr = req.app_data::<NewUser>().unwrap();

    match NewUser::create_user(
        conn,
        user.first_name.to_string(),
        user.last_name.to_string(),
        user.email.to_string(),
        user.pass.to_string(),
    ) {
        Ok(created_user) => HttpResponse::Ok().json(created_user),
        Err(_) => HttpResponse::Ok().body("Error creating new user"),
    }
}
*/

pub async fn handle(db: web::Data<db_utils::DbPool>, user: Json<NewUser>) -> impl Responder {
    match NewUser::create_user(
        &db.get().unwrap(),
        user.first_name.to_string(),
        user.last_name.to_string(),
        user.email.to_string(),
        user.pass.to_string(),
    ) {
        Ok(created_user) => HttpResponse::Ok().json(created_user),
        Err(_) => HttpResponse::Ok().body("Error creating new user"),
    }
}
