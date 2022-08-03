use actix_web::{
    web::{self, Json},
    HttpResponse, Responder,
};

use crate::{db_utils, models::loan::NewLoan};

pub async fn handle(db: web::Data<db_utils::DbPool>, loan: Json<NewLoan>) -> impl Responder {
    match NewLoan::create_loan(&db.get().unwrap(), loan.book_id, loan.user_id.to_string()) {
        Ok(created_loan) => HttpResponse::Ok().json(created_loan),
        Err(_) => HttpResponse::BadRequest().body("Error creating new loan"),
    }
}
