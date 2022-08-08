use actix_web::{
    web::{self, Json},
    HttpResponse, Responder, ResponseError,
};

use crate::{
    db_utils,
    models::loan::{Loan, NewLoan},
};

pub async fn handle(db: web::Data<db_utils::DbPool>) -> impl Responder {
    match Loan::get_all_loans(&db.get().unwrap()) {
        Ok(loans) => HttpResponse::Ok().json(loans),
        Err(e) => e.error_response(),
    }
}

pub async fn create_loan(db: web::Data<db_utils::DbPool>, loan: Json<NewLoan>) -> impl Responder {
    match NewLoan::create_loan(&db.get().unwrap(), loan.book_id, loan.user_id.to_string()) {
        Ok(created_loan) => HttpResponse::Ok().json(created_loan),
        Err(e) => e.error_response(),
    }
}

pub async fn return_loan(
    db: web::Data<db_utils::DbPool>,
    path: web::Path<(String)>,
) -> impl Responder {
    let loan_id = path.into_inner();

    match Loan::return_loan(&db.get().unwrap(), loan_id) {
        Ok(()) => HttpResponse::Ok().body("Successfully returned a loan!"),
        Err(e) => e.error_response(),
    }
}

pub async fn check_deadline_status(
    db: web::Data<db_utils::DbPool>,
    path: web::Path<(String)>,
) -> impl Responder {
    let loan_id = path.into_inner();

    match Loan::check_status(&db.get().unwrap(), loan_id) {
        Ok(status_msg) => HttpResponse::Ok().body(status_msg),
        Err(e) => e.error_response(),
    }
}
