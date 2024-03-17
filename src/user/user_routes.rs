// user/user_routes.rs
use crate::db::PgPool;
use crate::user::user_dtos::CreateUserReqDto;
use crate::user::user_repository::create_user;
use actix_web::{web, HttpResponse, Responder};

async fn create_user_route(
    pool: web::Data<PgPool>,
    user_dto: web::Json<CreateUserReqDto>,
) -> impl Responder {
    match create_user(&pool, user_dto.into_inner()).await {
        Ok(created_user) => HttpResponse::Created().json(created_user),
        Err(err) => {
            eprintln!("Failed to create user: {}", err);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/users").route("", web::post().to(create_user_route)));
}
