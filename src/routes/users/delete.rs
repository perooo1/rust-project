use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::user::User};

pub async fn handle(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let user_id = path.into_inner();
    match User::delete_user(&db.get().unwrap(), user_id) {
        Ok(cnt) => HttpResponse::Ok().json(cnt),
        Err(e) => e.error_response(),
    }
}
