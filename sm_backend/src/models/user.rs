use std::future::Future;
use std::pin::Pin;
use actix_web::{dev::Payload, FromRequest, HttpRequest, web};
use crate::auth;
use serde::{Serialize, Deserialize};
use crate::errors;
use errors::CustomError;
use crate::Pool;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub hashed_password: String,
    pub is_admin: bool,
    pub creation_datestamp: i64
}

impl FromRequest for User {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<User, CustomError>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let db = req.app_data::<web::Data<Pool>>().unwrap();
        auth::authenticate_request(&req, db.clone(), auth::AuthType::User)
    }
}
