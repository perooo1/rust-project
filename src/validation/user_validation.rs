use crate::{custom_errors, models::user::User};
use diesel::PgConnection;
use passwords::analyzer;
use validator::{self, ValidationError};

pub fn validate_email(email: &String) -> bool {
    let email = email.trim();
    validator::validate_email(email)
}

pub fn validate_password(password: &String) -> bool {
    let password = password.trim();

    let analyzed = analyzer::analyze(password);

    if analyzed.length() < 8
        || analyzed.uppercase_letters_count() == 0
        || analyzed.numbers_count() == 0
        || analyzed.spaces_count() != 0
    {
        println!("Password should be at least 8 chars long and contain one uppercase, one number and no spaces (' ')");
        return false;
    }

    true
}

pub fn user_exists(conn: &PgConnection, id: &String) -> bool {
    match User::get_user_by_id(conn, id) {
        Ok(_) => true,
        Err(_) => false,
    }
}
