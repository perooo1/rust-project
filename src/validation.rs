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
    validator::validate_email(email)
}

pub fn validate_password(password: &String) -> bool {
    let password = password.trim();

    let analyzed = analyzer::analyze(password);
    if analyzed.length() < 8 {
        println!("Password should be at least 8 chars long");
        return false
    } else if analyzed.uppercase_letters_count() == 0 {
        println!("Password should contain at least one uppercase letter");
        return false
    } else if analyzed.numbers_count() == 0 {
        println!("Password should have at least one number");
        return false
    } else if analyzed.spaces_count() != 0 {
        println!("Password should have any spaces");
        return false
    } 

    true
}
