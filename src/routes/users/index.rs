use crate::{db_utils, models::user::User};
use actix_web::ResponseError;
use actix_web::{web, HttpResponse, Responder};

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    let users = User::get_all_users(&db.get().unwrap());
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => e.error_response(),
    }
}
