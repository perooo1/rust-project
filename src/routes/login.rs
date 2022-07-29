use actix_web::{web, web::Json, HttpResponse, Responder};

use crate::{db_utils, models::authentication::auth::AuthUser};

pub async fn handle(db: web::Data<db_utils::DbPool>, user: Json<AuthUser>) -> impl Responder {
    match AuthUser::authenticate(
        &db.get().unwrap(),
        user.email.to_string(),
        user.pass.to_string(),
    ) {
        Ok((auth_user, token)) => HttpResponse::Ok()
            .append_header(("jwt", token))
            .json(auth_user),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
