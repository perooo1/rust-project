use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

///Custom error type used for error handling throughout application
#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "User/Loan/Book not found in database")]
    NotFound,
    #[display(fmt = "Error creating user in db")]
    UserCreationError,
    #[display(fmt = "Invalid user email or password")]
    InvalidCredentials,
    #[display(fmt = "Error hashing password")]
    PasswordHashError,
    #[display(fmt = "Internal server error, sth wrong with db")]
    InternalError,
    #[display(fmt = "Bad request")]
    BadRequest,
    #[display(fmt = "Loan already returned")]
    LoanReturnedError,
    #[display(fmt = "Book is already loaned")]
    BookLoanedError,
    #[display(fmt = "Error updating book loaned status")]
    BookLoanStatusError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidCredentials => StatusCode::BAD_REQUEST,
            AppError::PasswordHashError => StatusCode::UNAUTHORIZED,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::UserCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::LoanReturnedError => StatusCode::BAD_REQUEST,
            AppError::BookLoanedError => StatusCode::BAD_REQUEST,
            AppError::BookLoanStatusError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::InvalidCString(_) => AppError::BadRequest,
            diesel::result::Error::DatabaseError(_, _) => AppError::InternalError,
            diesel::result::Error::NotFound => AppError::NotFound,
            diesel::result::Error::QueryBuilderError(_) => AppError::BadRequest,
            diesel::result::Error::DeserializationError(_) => AppError::BadRequest,
            diesel::result::Error::SerializationError(_) => AppError::BadRequest,
            diesel::result::Error::RollbackTransaction => AppError::InternalError,
            diesel::result::Error::AlreadyInTransaction => AppError::InternalError,
            _ => AppError::InternalError,
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        match err {
            bcrypt::BcryptError::Io(_) => AppError::PasswordHashError,
            bcrypt::BcryptError::CostNotAllowed(_) => AppError::PasswordHashError,
            bcrypt::BcryptError::InvalidCost(_) => AppError::PasswordHashError,
            bcrypt::BcryptError::InvalidPrefix(_) => AppError::PasswordHashError,
            bcrypt::BcryptError::InvalidHash(_) => AppError::PasswordHashError,
            bcrypt::BcryptError::InvalidSaltLen(_) => AppError::PasswordHashError,
            bcrypt::BcryptError::InvalidBase64(_) => AppError::PasswordHashError,
            bcrypt::BcryptError::Rand(_) => AppError::PasswordHashError,
        }
    }
}
