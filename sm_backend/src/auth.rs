use actix_web::{HttpRequest, web};
use crate::Pool;

use crate::models::user::User;

use crate::errors;
use errors::CustomError;

pub enum AuthType {
    User,
    Admin
}

pub async fn authenticate_request(req: &HttpRequest, pool: &web::Data<Pool>, auth_type: AuthType) -> Result<User, CustomError> {
    match req.headers().get(actix_web::http::header::AUTHORIZATION) {
        Some(header) => {
            // TODO: Is there a way to make this only make one DB fetch? Right now it makes two (one to get the token's associate username, a second to get the user itself)
            // TODO: remove this unwrap
            let token = header.to_str().unwrap();
            let record = sqlx::query!(r#"SELECT username FROM tokens WHERE token = $1"#, token).fetch_one(pool.as_ref()).await
                .map_err(|_| CustomError {error_type: errors::ErrorType::BadClientData, message: Some(format!("invalid token in authorization header"))})?;
            let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE username = $1"#, record.username).fetch_one(pool.as_ref()).await
                .map_err(|_| CustomError {error_type: errors::ErrorType::InternalError, message: None})?;
            if user.id.is_none() {
                return Err(CustomError {error_type: errors::ErrorType::InternalError, message: Some(format!("User does not have an ID"))})
            }
            match auth_type {
                AuthType::User => Ok(user),
                AuthType::Admin => {
                    match user.is_admin {
                        true => Ok(user),
                        false => Err(CustomError {error_type: errors::ErrorType::InvalidAuth, message: Some(format!("admin authorization required"))})
                    }
                }
            }
        },
        None => Err(CustomError {error_type: errors::ErrorType::BadClientData, message: Some(format!("missing authorization header"))})
    }
}
