use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SqlxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DbError(String),
    //ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error_message: String,
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DbError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            // EzyTutorError::ActixError(msg) => {
            //     println!("Server error occured: {:?}", msg);
            //     "Internal server error".into()
            // }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occured: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl fmt::Display for EzyTutorError {
    fn fmt (&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<SqlxError> for EzyTutorError {
    fn from(err: SqlxError) -> Self {
        EzyTutorError::DbError(err.to_string())
    }
}

impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DbError(_) /*| EzyTutorError::ActixError(_)*/ => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error_message: self.error_response(),
        })
    }
}