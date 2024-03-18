// main.rs
mod db;
mod user;
mod db_config;

extern crate env_logger;
// use env_logger::Builder;
// use log::LevelFilter;

use actix_web::{web, App, HttpServer, HttpResponse}; //middleware::Logger
use user::user_routes;
use std::io;

use crate::db::create_pool;

// use tokio::time::{sleep, Duration};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Builder::from_default_env()
        // .filter_level(LevelFilter::Debug)
        // .init();

    let pool = create_pool().await;

    // let pool_clone = pool.clone();

    // tokio::spawn(async move {
    //     loop {
    //         let status = pool_clone.status();
    //         println!("Available connections: {}", status.available);
    //         println!("Max connections: {}", status.max_size);
    //         println!("Total connections: {}", status.size);
    //         println!("Waiting connections: {}", status.waiting);
    //         // Aguardar 5 segundos antes de imprimir os status novamente
    //         sleep(Duration::from_secs(5)).await;
    //     }
    // });

    HttpServer::new(move || {
        App::new()
            // .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(user_routes::configure_routes)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}