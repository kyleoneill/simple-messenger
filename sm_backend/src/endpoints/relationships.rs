use actix_web::{get, post, web, HttpResponse, Responder, dev::HttpServiceFactory, HttpRequest};
use serde::{Serialize, Deserialize};
use sqlx::sqlite::SqliteQueryResult;
use crate::Pool;
use crate::models::user_relationship::UserRelationship;

use crate::errors;
use errors::{CustomError, ErrorType};
use crate::models::user::User;

#[derive(Deserialize)]
pub struct TargetUsername {
    target_username: String
}

#[derive(Serialize, Deserialize)]
pub struct PublicFacingRelationship {
    pub username: String,
    pub is_friend: bool,
    pub is_blocked: bool
}

// TODO: Make this paginated, taking query params to determine page size, starting page, pages to return
#[get("")]
pub async fn get_relationships(_req: HttpRequest, pool: web::Data<Pool>, user: User) -> Result<impl Responder, CustomError> {
    match get_relationships_sql(&pool, user.id.unwrap()).await {
        Ok(relationships) => {
            Ok(web::Json(relationships))
        }
        Err(_e) => Err(CustomError{error_type: ErrorType::InternalError, message: None})
    }
}

#[get("/with_user")]
pub async fn get_relationship_with_user(_req: HttpRequest, pool: web::Data<Pool>, user: User, target_username: web::Query<TargetUsername>) -> Result<impl Responder, CustomError> {
    if user.username == target_username.target_username {
        return Err(CustomError::new( ErrorType::BadClientData, Some("You are not a friend with yourself.".to_string())))
    }
    match get_single_relationship_sql(&pool, user.id.unwrap(), &target_username.target_username).await {
        Ok(relationship) => Ok(web::Json(relationship)),
        Err(sqlx::Error::RowNotFound) => Err(CustomError::new(ErrorType::NotFound,Some("Relationship with target_username not found".to_string()))),
        _ => Err(CustomError::new(ErrorType::InternalError, None))
    }
}

#[post("/add_friend")]
/// Add a friend
pub async fn add_friend_endpoint(_req: HttpRequest, pool: web::Data<Pool>, user: User, friend_request: web::Json<TargetUsername>) -> Result<impl Responder, CustomError> {
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
