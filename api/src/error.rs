use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("认证错误: {0}")]
    Auth(String),

    #[error("权限不足: {0}")]
    Forbidden(String),

    #[error("请求无效: {0}")]
    BadRequest(String),

    #[error("资源未找到: {0}")]
    NotFound(String),

    #[error("数据库错误: {0}")]
    Database(String),

    #[error("验证错误: {0}")]
    Validation(String),

    #[error("服务器内部错误: {0}")]
    Internal(String),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Auth(_) => StatusCode::UNAUTHORIZED,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Error::BadRequest(_) | Error::Validation(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Database(_) | Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut result = HashMap::new();
        result.insert("error", self.to_string());
        HttpResponse::build(self.status_code()).json(result)
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(err: sea_orm::DbErr) -> Self {
        Error::Database(err.to_string())
    }
}

impl From<actix_web::Error> for Error {
    fn from(err: actix_web::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(err: bcrypt::BcryptError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Error::Auth(err.to_string())
    }
}

impl From<actix_web_validator::Error> for Error {
    fn from(err: actix_web_validator::Error) -> Self {
        Error::Validation(err.to_string())
    }
}
