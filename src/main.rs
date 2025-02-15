//! Main entry point into the Case Transformer server application
use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use case_transformer_rs::alive;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    init_server().await
}

pub async fn init_server() -> eyre::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

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
    .bind(("127.0.0.1", 5000))?
    .run();

    server.await?;

    Ok(())
}

