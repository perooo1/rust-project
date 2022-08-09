use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::{db_utils, models::loan::Loan};

pub async fn handle(db: web::Data<db_utils::DbPool>, path: web::Path<(String)>) -> impl Responder {
    let loan_id = path.into_inner();

    match Loan::check_status(&db.get().unwrap(), loan_id) {
        Ok(status_msg) => HttpResponse::Ok().body(status_msg),
        Err(e) => e.error_response(),
    }
}
