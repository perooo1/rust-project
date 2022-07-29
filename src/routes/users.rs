use crate::models::user::NewUser;
use crate::{db_utils, models::user::User};
use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    let users = User::get_all_users(&db.get().unwrap());
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::Ok().body("Error getting users"),
    }
}

pub async fn del_usr(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let user_id = path.into_inner();
    match User::delete_user(&db.get().unwrap(), user_id){
        Ok(cnt) => HttpResponse::Ok().json(cnt),
        Err(_) => HttpResponse::Ok().body("Error deleting user witha provided id")


    }

}
