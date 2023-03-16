use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, error, http::header, middleware};
use actix_cors::Cors;
mod errors;
mod util;
mod auth;
mod models;
mod endpoints;

async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type(header::ContentType::json())
        .body(format!(r#"{{"status":"Online"}}"#))
}

pub type Database = sqlx::Sqlite;
pub type Pool = sqlx::Pool<Database>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = Pool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(endpoints::users::controller())
                    .service(endpoints::relationships::controller())
                    .route("/health", web::get().to(health))
            )
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"msg":"Internal server error"}}"#))
                )
                .into()
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
