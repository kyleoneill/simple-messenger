use actix_web::http::StatusCode;
use actix_web::{http::header::ContentType, get, post, put, web, HttpResponse, Responder, dev::HttpServiceFactory, HttpRequest};
use serde::{Serialize, Deserialize};
use sqlx::sqlite::SqliteQueryResult;
use crate::Pool;
use crate::users;

use crate::util;
use crate::auth;

use crate::errors;
use errors::{CustomError, ErrorType};

#[derive(Deserialize)]
pub struct FriendRequest {
    target_username: String
}

#[post("/send_friend_request")]
pub async fn send_friend_request_endpoint(req: HttpRequest, pool: web::Data<Pool>, friend_request: web::Json<FriendRequest>) -> Result<impl Responder, CustomError> {
    let user = auth::authenticate_request(&req, &pool, auth::AuthType::User).await?;
    let target_user = users::get_user_by_username(&pool, &friend_request.target_username).await
        .map_err(|_| CustomError {error_type: ErrorType::NotFound, message: Some(format!("User not found with username {}", &friend_request.target_username))})?;
    if user.id == target_user.id {
        return Err(CustomError {error_type: ErrorType::BadClientData, message: Some("You cannot add yourself as a friend".to_string())})
    }
    // TODO: need to make sure that a relationship doesn't already exist
    match create_friend_request_sql(&pool, &user.username, &target_user.username).await {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(e) => {
            println!("{}", e);
            Err(CustomError {error_type: ErrorType::InternalError, message: None})
        }
    }
}

#[get("/get_friend_requests")]
pub async fn get_friend_requests_endpoint(req: HttpRequest, pool: web::Data<Pool>) -> Result<impl Responder, CustomError> {
    Ok(HttpResponse::Ok())
}

#[put("/accept_friend_request")]
pub async fn accept_friend_request_endpoint(req: HttpRequest, pool: web::Data<Pool>) -> Result<impl Responder, CustomError> {
    Ok(HttpResponse::Ok())
}

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/relationships")
        .service(send_friend_request_endpoint)
        .service(get_friend_requests_endpoint)
        .service(accept_friend_request_endpoint)
}

async fn create_friend_request_sql(pool: &web::Data<Pool>, source_username: &str, requested_username: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO friendRequests (source_user_id, target_user_id)
        SELECT user1.id, user2.id
        FROM users user1, users user2
        WHERE user1.username = $1 and user2.username = $2
        "#,
        source_username,
        requested_username
    ).execute(pool.as_ref()).await
}
