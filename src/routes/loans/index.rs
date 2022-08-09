use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::loan::Loan};

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    match Loan::get_all_loans(&db.get().unwrap()) {
        Ok(loans) => HttpResponse::Ok().json(loans),
        Err(e) => e.error_response(),
    }
}
