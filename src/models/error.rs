use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    DatabaseError(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<postgres::Error> for AppError {
    fn from(err: postgres::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(format!("Invalid JSON: {}", err))
    }
}
