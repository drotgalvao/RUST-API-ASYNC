// main.rs
mod db;
mod user;
mod db_config;

extern crate env_logger;
use env_logger::Builder;
use log::LevelFilter;

use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use user::user_routes;
use std::io;

use crate::db::create_pool;


#[actix_web::main]
async fn main() -> io::Result<()> {
    Builder::from_default_env()
        .filter_level(LevelFilter::Debug)
        .init();

    let pool = create_pool().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(user_routes::configure_routes)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}