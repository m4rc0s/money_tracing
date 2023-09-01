use actix_web::{App, HttpServer};

use cashflow::api::*;

pub mod cashflow;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register_movement)})
            .bind(("127.0.0.1", 9000))?
            .run()
            .await
    }
