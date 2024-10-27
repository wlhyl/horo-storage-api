use std::{collections::HashMap, fmt};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {
    ActixError(String),
    DBError(String),
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
    Forbidden(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Error::ActixError(s) => s,
            Error::DBError(s) => s,
            Error::BadRequest(s) => s,
            Error::NotFound(s) => s,
            Error::InternalServerError(s) => s,
            Error::Forbidden(s) => s,
        };
        write!(f, "{}", s)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ActixError(_) | Error::DBError(_) | Error::InternalServerError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::BadRequest(_) | Error::NotFound(_) => StatusCode::BAD_REQUEST,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut result = HashMap::new();
        result.insert("error", format!("{}", self));
        HttpResponse::build(self.status_code()).json(result)
    }
}

impl From<actix_web::error::Error> for Error {
    fn from(error: actix_web::error::Error) -> Self {
        Error::ActixError(error.to_string())
    }
}

impl From<sea_orm::error::DbErr> for Error {
    fn from(err: sea_orm::error::DbErr) -> Self {
        Error::DBError(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(error: bcrypt::BcryptError) -> Self {
        Error::InternalServerError(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Error::InternalServerError(error.to_string())
    }
}

impl From<actix_web_validator::Error> for Error {
    fn from(value: actix_web_validator::Error) -> Self {
        Error::BadRequest(value.to_string())
    }
}
