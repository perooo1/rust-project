use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::loan::Loan};

pub async fn handle(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let user_id = path.into_inner();
    match Loan::get_loans_for_single_user(&db.get().unwrap(), user_id) {
        Ok(loans) => HttpResponse::Ok().json(loans),
        Err(e) => e.error_response(),
    }
}
