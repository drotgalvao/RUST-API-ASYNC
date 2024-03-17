// db.rs
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

pub type PgPool = Pool;

pub async fn create_pool() -> PgPool {
    let mut cfg = Config::new();
    cfg.dbname = Some("mydatabase".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.host = Some("postgres".to_string()); // localhost or postgres
    cfg.port = Some(5432); // Defina a porta explicitamente

    match cfg.create_pool(Some(Runtime::Tokio1), NoTls) {
        Ok(pool) => {
            println!("Connection pool created successfully.");
            pool
        },
        Err(e) => {
            eprintln!("Failed to create connection pool: {}", e);
            panic!("Failed to create connection pool");
        }
    }
}