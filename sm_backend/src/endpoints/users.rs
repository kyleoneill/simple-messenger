use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder, dev::HttpServiceFactory};
use serde::Deserialize;
use sqlx::sqlite::SqliteQueryResult;
use crate::Pool;
extern crate bcrypt;

use crate::util;
use crate::models::user::User;

use crate::errors;
use errors::{CustomError, ErrorType};

#[derive(Deserialize)]
pub struct WebUser {
    username: String,
    password: String
}

#[post("")]
/// Takes a username and password and creates a new user. This will 409 if the username is
/// already in use
///
/// | Request Body | Description |
/// | ------------ | ----------- |
/// | *username*   | The username of the new account |
/// | *password*   | The password of the new account |
pub async fn post_user(user: web::Json<WebUser>, pool: web::Data<Pool>) -> Result<impl Responder, CustomError> {
    match get_user_by_username(&pool, &user.username).await {
        Ok(_user) => Err(CustomError {error_type: ErrorType::AlreadyExists, message: Some(format!("That username is already in use"))}),
        Err(_) => {
            match create_user(&pool, user, false).await {
                Ok(_) => Ok(HttpResponse::Created()),
                Err(_) => Err(CustomError {error_type: ErrorType::InternalError, message: None})
            }
        }
    }
}

#[post("/auth")]
/// Takes a username and password and authenticates new user. This will 400 if the credentials
/// are incorrect of the user does not exist.
///
/// | Request Body | Description |
/// | ------------ | ----------- |
/// | *username*   | The username of the new account |
/// | *password*   | The password of the new account |
pub async fn auth_user(pool: web::Data<Pool>, user: web::Json<WebUser>) -> Result<impl Responder, CustomError> {
    match validate_user_token(&pool, &user).await {
        Ok(are_credentials_valid) => match are_credentials_valid {
            true => {
                match generate_token(&pool, &user.username).await {
                    Ok(token) => Ok(HttpResponse::Created().content_type(ContentType::json()).body(format!(r#"{{"token":"{}"}}"#, &token))),
                    Err(_) => Err(CustomError {error_type: ErrorType::InternalError, message: None})
                }
            },
            false => Err(CustomError {error_type: ErrorType::BadClientData, message: Some("Invalid credentials".to_owned())})
        },
        Err(_) => Err(CustomError {error_type: ErrorType::BadClientData, message: Some("User does not exist".to_owned())})
    }
}

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/users")
        .service(post_user)
        .service(auth_user)
}

pub async fn get_user_by_username(pool: &web::Data<Pool>, username: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE username = $1
        "#,
        username
    ).fetch_one(pool.as_ref()).await
}

async fn validate_user_token(pool: &web::Data<Pool>, web_user: &web::Json<WebUser>) -> Result<bool, sqlx::Error> {
    match sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE username = $1
        "#,
        web_user.username
    ).fetch_one(pool.as_ref()).await {
        Ok(user) => Ok(bcrypt::verify(&web_user.password, &user.hashed_password).unwrap()),
        Err(e) => Err(e)
    }
}

async fn generate_token(pool: &web::Data<Pool>, username: &str) -> Result<String, sqlx::Error> {
    let token = util::random_string(25); // TODO - replace this with real token generation
    let current_time = util::get_unix_time();
    match sqlx::query!(
        r#"
        REPLACE INTO tokens (token, username, creation_time) VALUES ($1, $2, $3)
        "#,
        token,
        username,
        current_time
    ).execute(pool.as_ref()).await {
        Ok(_) => Ok(token),
        Err(e) => Err(e)
    }
}

async fn create_user(pool: &web::Data<Pool>, user: web::Json<WebUser>, is_admin: bool) -> Result<SqliteQueryResult, sqlx::Error> {
    let hashed_password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).expect("Failed to hash password");
    sqlx::query!(
        r#"
        INSERT INTO users (username, hashed_password, is_admin) VALUES ($1, $2, $3)
        "#,
        user.username,
        hashed_password,
        is_admin
    ).execute(pool.as_ref()).await
}
