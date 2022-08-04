use crate::models::loan::Loan;
use crate::models::user::NewUser;
use crate::{db_utils, models::user::User};
use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};
//use crate::db_utils::AppState;

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    let users = User::get_all_users(&db.get().unwrap());
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::Ok().body("Error getting users"),
    }
}

/*
pub async fn handle(db: web::Data<AppState>) -> impl Responder {
    let users = User::get_all_users(&db.get_connection());
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::Ok().body("Error getting users"),
    }
}
*/

pub async fn del_usr(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let user_id = path.into_inner();
    match User::delete_user(&db.get().unwrap(), user_id) {
        Ok(cnt) => HttpResponse::Ok().json(cnt),
        Err(_) => HttpResponse::Ok().body("Error deleting user witha provided id"),
    }
}

pub async fn get_loans_for_single_user(
    db: web::Data<db_utils::DbPool>,
    path: web::Path<(String)>,
) -> impl Responder {
    let user_id = path.into_inner();
    match Loan::get_loans_for_single_user(&db.get().unwrap(), user_id) {
        Ok(loans) => HttpResponse::Ok().json(loans),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
