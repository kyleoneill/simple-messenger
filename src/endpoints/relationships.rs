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

pub struct UserRelationship {
    pub user_one_id: i64,
    pub user_two_id: i64,
    pub is_friend: bool,
    pub is_blocked: bool
}

pub struct Relationship {
    pub user_one: UserRelationship,
    pub user_two: UserRelationship
}

impl Relationship {
    fn users_are_friends(&self) -> bool {
        self.user_one.is_friend && self.user_two.is_friend
    }
}

#[post("/add_friend")]
pub async fn add_friend_endpoint(req: HttpRequest, pool: web::Data<Pool>, friend_request: web::Json<FriendRequest>) -> Result<impl Responder, CustomError> {
    let user = auth::authenticate_request(&req, &pool, auth::AuthType::User).await?;
    if user.username == friend_request.target_username {
        return Err(CustomError {error_type: ErrorType::BadClientData, message: Some("You cannot add yourself as a friend".to_string())})
    }
    match add_friend_sql(&pool, &user.username, &friend_request.target_username).await {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(e) => {
            println!("{}", e);
            Err(CustomError {error_type: ErrorType::InternalError, message: None})
        }
    }
}

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/relationships")
        .service(add_friend_endpoint)
}

async fn add_friend_sql(pool: &web::Data<Pool>, source_username: &str, requested_username: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO userRelationship
        SELECT user1.id, user2.id, 1, 0
        FROM users user1, users user2
        WHERE user1.username = $1 and user2.username = $2
        "#,
        source_username,
        requested_username
    ).execute(pool.as_ref()).await
}

async fn get_relationship_by_usernames_sql(pool: &web::Data<Pool>, source_username: &str, requested_username: &str) -> Result<Relationship, CustomError> {
    let first_relationship = sqlx::query_as!(
        UserRelationship,
        r#"
        SELECT user_one_id, user_two_id, is_friend, is_blocked FROM userRelationship
        INNER JOIN users
        ON userRelationship.user_one_id = users.id
        WHERE users.username = $1 AND userRelationship.user_two_id IN
        (SELECT id FROM users WHERE username = $2)
        "#,
        source_username,
        requested_username
    ).fetch_one(pool.as_ref()).await
        .map_err(|_| CustomError {error_type: errors::ErrorType::NotFound, message: None})?;
    let second_relationship = sqlx::query_as!(
        UserRelationship,
        r#"
        SELECT user_one_id, user_two_id, is_friend, is_blocked FROM userRelationship
        INNER JOIN users
        ON userRelationship.user_one_id = users.id
        WHERE users.username = $2 AND userRelationship.user_two_id IN
        (SELECT id FROM users WHERE username = $1)
        "#,
        source_username,
        requested_username
    ).fetch_one(pool.as_ref()).await
        .map_err(|_| CustomError {error_type: errors::ErrorType::NotFound, message: None})?;
    Ok(Relationship{ user_one: first_relationship, user_two: second_relationship })
}
