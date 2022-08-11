use crate::{custom_errors, models::user::User};
use diesel::PgConnection;
use passwords::{analyzer, AnalyzedPassword};
use validator::{self, ValidationError};

///Check if user's email is valid
pub fn validate_email(email: &String) -> bool {
    let email = email.trim();
    validator::validate_email(email)
}
///Check if user's password is valid
pub fn validate_password(password: &String) -> bool {
    let password = password.trim();

    let analyzed = analyzer::analyze(password);

    let length = check_password_length(&analyzed);
    let uppercase = check_uppercase(&analyzed);
    let spaces = check_spaces(&analyzed);

    if length || uppercase || spaces {
        println!("Password should be at least 8 chars long and contain one uppercase, one number and no spaces (' ')");
        return false;
    }

    true
}

fn check_password_length(a: &AnalyzedPassword) -> bool {
    return a.length() < 8;
}

fn check_uppercase(a: &AnalyzedPassword) -> bool {
    return a.uppercase_letters_count() == 0;
}

fn check_spaces(a: &AnalyzedPassword) -> bool {
    return a.spaces_count() != 0;
}

///Check if user exists in postgres database
pub fn user_exists(conn: &PgConnection, id: &String) -> bool {
    match User::get_user_by_id(conn, id) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use passwords::analyzer;

    use super::{check_password_length, check_spaces, check_uppercase, validate_password};

    #[test]
    fn test_password_not_long_enough() {
        let a = analyzer::analyze("petar");
        assert_eq!(check_password_length(&a), true)
    }

    #[test]
    fn test_password_no_uppercase() {
        let a = analyzer::analyze("ptar");
        assert_eq!(check_uppercase(&a), true)
    }

    #[test]
    fn test_password_with_spaces() {
        let a = analyzer::analyze("p tar");
        assert_eq!(check_spaces(&a), true)
    }

    #[test]
    fn test_password() {
        assert_eq!(validate_password(&String::from("PppPetar123")), true)
    }
}
