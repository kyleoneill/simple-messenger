use actix_web::{get, post, web, HttpResponse, Responder, dev::HttpServiceFactory, HttpRequest};
use serde::{Serialize, Deserialize};
use sqlx::sqlite::SqliteQueryResult;
use crate::Pool;

use crate::auth;

use crate::errors;
use errors::{CustomError, ErrorType};

#[derive(Deserialize)]
pub struct TargetUsername {
    target_username: String
}

pub struct UserRelationship {
    pub user_one_id: i64,
    pub user_two_id: i64,
    pub is_friend: bool,
    pub is_blocked: bool
}

#[derive(Serialize, Deserialize)]
pub struct PublicFacingRelationship {
    pub username: String,
    pub is_friend: bool,
    pub is_blocked: bool
}

#[get("")]
pub async fn get_relationships(req: HttpRequest, pool: web::Data<Pool>) -> Result<impl Responder, CustomError> {
    let user = auth::authenticate_request(&req, &pool, auth::AuthType::User).await?;
    match get_relationships_sql(&pool, user.id.unwrap()).await {
        Ok(relationships) => {
            Ok(web::Json(relationships))
        }
        Err(_e) => Err(CustomError{error_type: ErrorType::InternalError, message: None})
    }
}

#[get("/with_user")]
pub async fn get_relationship_with_user(req: HttpRequest, pool: web::Data<Pool>, target_username: web::Query<TargetUsername>) -> Result<impl Responder, CustomError> {
    let user = auth::authenticate_request(&req, &pool, auth::AuthType::User).await?;
    if user.username == target_username.target_username {
        return Err(CustomError {error_type: ErrorType::BadClientData, message: Some("You are not a friend with yourself.".to_string())})
    }
    match get_single_relationship_sql(&pool, user.id.unwrap(), &target_username.target_username).await {
        Ok(relationship) => {
            Ok(web::Json(relationship))
        }
        Err(sqlx_err) => {
            match sqlx_err {
                sqlx::Error::RowNotFound => Err(CustomError {error_type: ErrorType::NotFound, message: Some("Relationship with target_username not found".to_string())}),
                _ => Err(CustomError {error_type: ErrorType::InternalError, message: None})
            }
        }
    }
}

#[post("/add_friend")]
/// Add a friend
pub async fn add_friend_endpoint(req: HttpRequest, pool: web::Data<Pool>, friend_request: web::Json<TargetUsername>) -> Result<impl Responder, CustomError> {
    let user = auth::authenticate_request(&req, &pool, auth::AuthType::User).await?;
    if user.username == friend_request.target_username {
        return Err(CustomError {error_type: ErrorType::BadClientData, message: Some("You cannot add yourself as a friend".to_string())})
    }
    match add_friend_sql(&pool, &user.username, &friend_request.target_username).await {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(_e) => {
            // TODO: Log this error?
            Err(CustomError {error_type: ErrorType::InternalError, message: None})
        }
    }
}

pub fn controller() -> impl HttpServiceFactory {
    web::scope("/relationships")
        .service(add_friend_endpoint)
        .service(get_relationships)
        .service(get_relationship_with_user)
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

async fn get_relationships_sql(pool: &web::Data<Pool>, user_id: i64) -> Result<Vec<PublicFacingRelationship>, sqlx::Error> {
    sqlx::query_as!(
        PublicFacingRelationship,
        r#"
        SELECT users.username, userRelationship.is_friend, userRelationship.is_blocked
        FROM userRelationship INNER JOIN users
        ON userRelationship.user_two_id = users.id
        WHERE userRelationship.user_one_id = $1
        "#,
        user_id
    ).fetch_all(pool.as_ref()).await
}

async fn get_single_relationship_sql(pool: &web::Data<Pool>, user_id: i64, target_username: &str) -> Result<PublicFacingRelationship, sqlx::Error> {
    sqlx::query_as!(
        PublicFacingRelationship,
        r#"
        SELECT users.username, userRelationship.is_friend, userRelationship.is_blocked
        FROM userRelationship INNER JOIN users
        ON userRelationship.user_two_id = users.id
        WHERE userRelationship.user_one_id = $1 AND users.username = $2
        "#,
        user_id,
        target_username
    ).fetch_one(pool.as_ref()).await
}
