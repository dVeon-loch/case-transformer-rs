//! Main entry point into the Case Transformer server application
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use case_transformer_rs::alive;

const SERVER_PORT: u16 = 5000;

const LOG_LEVEL: &'static str = "debug";

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    init_server().await
}

/// Sets up the logging for the application
pub fn init_logs() {
    std::env::set_var("RUST_LOG", LOG_LEVEL);
    env_logger::init();
}

/// Contains the logic required to initiate the actix webserver
/// and set up the endpoints
pub async fn init_server() -> eyre::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .max_age(3600), // Cache preflight request for 1 hour
            )
            .service(web::scope("/api/v1").service(alive))
    })
    .bind(("127.0.0.1", SERVER_PORT))?
    .run();

    server.await?;

    Ok(())
}

