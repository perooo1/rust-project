use crate::models::user::NewUser;
use crate::{db_utils, models::user::User};
use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
/* 
    let users = match User::get_all_users(&db.get().unwrap()) {
        Ok(_users) => HttpResponse::Ok().json(_users),
        Err(_) => HttpResponse::Ok().body("Error getting users"),
    };
*/

    let users = User::get_all_users(&db.get().unwrap());
    match users{
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::Ok().body("Error getting users"),
    }


}
