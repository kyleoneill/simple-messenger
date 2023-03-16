use std::future::Future;
use std::pin::Pin;
use actix_web::{HttpRequest, web};
use crate::Pool;

use crate::models::user::User;

use crate::errors;
use errors::CustomError;

pub enum AuthType {
    User,
    Admin
}

// This function is especially cursed because it needs to be called by the non-async from_request fn when implementing FromRequest
// It needs to return a future rather than a Result as the returned data cannot be awaited within a sync function and there is an async db call here
// Most of this function is in an async block. This is so we can segment out the part that will be polled, so we do not need to
//   maintain references to all of the parameters, like req. The pool param and an owned copy of the header auth token
//   are moved into the async block
// The data from the async block is boxed because we must return an owned value. Async blocks are an "un-namable type"
//   these are types that exist but cannot be referred to by name. We need to refer to the block by name
//   in from_request when defining the Future type. Boxing something makes it so that you can own a value without knowing its size
//   which is needed when converting to a dyn Future, as they are unsized. A Box<dyn Trait> is namable
// Futures need to be pinned in order to be polled, so we need to wrap that box into a Pin
//   when something is pinned, we are making a promise that it will not be moved in memory until dropped
pub fn authenticate_request(req: &HttpRequest, pool: web::Data<Pool>, auth_type: AuthType) -> Pin<Box<dyn Future<Output = Result<User, CustomError>>>> {
    match req.headers().get(actix_web::http::header::AUTHORIZATION) {
        Some(header) => {
            // TODO: Is there a way to make this only make one DB fetch? Right now it makes two (one to get the token's associate username, a second to get the user itself)
            // TODO: remove this unwrap?
            let token = header.to_str().unwrap().to_string();
            let res = Box::new(async move {
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
            });
            Box::into_pin(res)
        },
        None => {
            let res = Box::new(async { Err(CustomError {error_type: errors::ErrorType::BadClientData, message: Some(format!("missing authorization header"))}) });
            Box::into_pin(res)
        }
    }
}
