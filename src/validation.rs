//validate user email and password

use crate::errors;
use passwords::analyzer;
use validator::{self, ValidationError};
/*
pub fn validate_email(email: &String) -> Result<(), ValidationError> {
    let is_email_valid = validator::validate_email(email);

    if !is_email_valid {
        return Err(ValidationError::new("Error validating user email"));
    }
    Ok(())

}

*/

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

//validate title/ author/ publisher is empty

pub fn is_book_author_empty(author: &String) -> bool{
    author.is_empty()
}
