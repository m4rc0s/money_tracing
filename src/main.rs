mod cashflow;

use actix_web::{App, HttpServer, web};

use cashflow::api::*;
use actix_web::middleware::Logger;
use sqlx::postgres::{PgPool, PgPoolOptions};

struct AppState {
    db: PgPool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env must be set");

    let db_connection_pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url).await {
            Ok(pool) => {
                pool
            }
            Err(_) => {
                std::process::exit(1);
            }
        };

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(AppState { db: db_connection_pool.clone() } ))
            .service(register_movement)})
            .bind(("127.0.0.1", 9000))?
            .run()
            .await
    }
