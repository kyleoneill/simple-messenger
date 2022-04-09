use core::fmt;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use actix_web::{error, HttpResponse, http::{StatusCode, header}};

fn format_error_message(f: &mut fmt::Formatter<'_>, message: &String) -> fmt::Result {
    write!(f, r#"{{"msg":"{message}"}}"#)
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum ErrorType {
    InternalError,
    BadClientData,
    NotFound,
    AlreadyExists,
    InvalidAuth
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct CustomError {
    pub error_type: ErrorType,
    pub message: Option<String>
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(message) => format_error_message(f, message),
            None => match self.error_type {
                ErrorType::InternalError => format_error_message(f, &"internal error".to_string()),
                ErrorType::BadClientData => format_error_message(f, &"bad request".to_string()),
                ErrorType::NotFound => format_error_message(f, &"resource not found".to_string()),
                ErrorType::AlreadyExists => format_error_message(f, &"resource already exists".to_string()),
                ErrorType::InvalidAuth => format_error_message(f, &"invalid authentication".to_string())
            }
        }
    }
}

impl error::ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            ErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadClientData => StatusCode::BAD_REQUEST,
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::AlreadyExists => StatusCode::CONFLICT,
            ErrorType::InvalidAuth => StatusCode::FORBIDDEN
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type(header::ContentType::json())
            .body(self.to_string())
    }
}
