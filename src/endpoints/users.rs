use actix_web::{http::header::ContentType, post, web, HttpResponse, Responder, dev::HttpServiceFactory, HttpRequest};
use serde::{Serialize, Deserialize};
use sqlx::sqlite::SqliteQueryResult;
use crate::Pool;
extern crate bcrypt;

use crate::util;

use crate::errors;
use errors::{CustomError, ErrorType};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub hashed_password: String,
    pub is_admin: bool
}

#[derive(Deserialize)]
pub struct WebUser {
    username: String,
    password: String
}

#[post("")]
pub async fn post_user(user: web::Json<WebUser>, pool: web::Data<Pool>) -> Result<impl Responder, CustomError> {
    match get_user_by_username(&pool, &user.username).await {
        Ok(_user) => Err(CustomError {error_type: ErrorType::AlreadyExists, message: Some(format!("That username is already in use"))}),
        Err(_) => {
            match create_user(&pool, user).await {
                Ok(_) => Ok(HttpResponse::Created()),
                Err(_) => Err(CustomError {error_type: ErrorType::InternalError, message: None})
            }
        }
    }
}

#[post("/auth")]
pub async fn auth_user(pool: web::Data<Pool>, user: web::Json<WebUser>) -> Result<impl Responder, CustomError> {
    match verify_user(&pool, &user).await {
        Ok(valid_credentials) => match valid_credentials {
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

#[post("/verify_token")]
pub async fn verify_token_endpoint(req: HttpRequest, pool: web::Data<Pool>) -> Result<impl Responder, CustomError> {
    match req.headers().get(actix_web::http::header::AUTHORIZATION) {
        Some(header) => {
            let token = header.to_str().unwrap();
            match verify_token_db(&pool, &token).await {
                Ok(_) => Ok(HttpResponse::Ok()),
                Err(err) => Err(err)
            }
        },
        None => Err(CustomError {error_type: ErrorType::BadClientData, message: Some(format!("missing authorization header"))})
    }
}

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/users")
        .service(post_user)
        .service(auth_user)
        .service(verify_token_endpoint)
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

async fn verify_user(pool: &web::Data<Pool>, web_user: &web::Json<WebUser>) -> Result<bool, sqlx::Error> {
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

async fn verify_token_db(pool: &web::Data<Pool>, token: &str) -> Result<(), CustomError> {
    match sqlx::query!(r#"SELECT username FROM tokens WHERE token = $1"#, token).fetch_one(pool.as_ref()).await {
        Ok(_record) => Ok(()),
        Err(_) => Err(CustomError {error_type: ErrorType::BadClientData, message: Some(format!("invalid token in authorization header"))})
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

async fn create_user(pool: &web::Data<Pool>, user: web::Json<WebUser>) -> Result<SqliteQueryResult, sqlx::Error> {
    let hashed_password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).expect("Failed to hash password");
    sqlx::query!(
        r#"
        INSERT INTO users (username, hashed_password) VALUES ($1, $2)
        "#,
        user.username,
        hashed_password
    ).execute(pool.as_ref()).await
}
