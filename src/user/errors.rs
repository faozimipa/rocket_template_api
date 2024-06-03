use core::fmt;
use std::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum CustomError {
    UserNotFound(u16),
    UserAlreadyExists(u16),
    EmailAlreadyExists(u16),
    MissingFields(u16, String),
    GenericError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::UserNotFound(code) => write!(f, "{} {}", code, "User not found"),
            CustomError::UserAlreadyExists(code) => write!(f, "{} {}", code, "User already exists"),
            CustomError::EmailAlreadyExists(code) => write!(f, "{} {}", code, "Email already exists"),
            CustomError::MissingFields(code, msg) => write!(f, "{} {} {}", code, "The following fields are missing: {}", msg),
            CustomError::GenericError(msg) => write!(f, "An error ocurred: {}", msg),
        }
    }
}

impl From<mongodb::error::Error> for CustomError {
    fn from(err: mongodb::error::Error) -> Self {
        CustomError::GenericError(err.to_string())
    }
}

impl Error for CustomError {}