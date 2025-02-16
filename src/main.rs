//! Main entry point into the Case Transformer server application
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use case_transformer_rs::endpoints::{alive, transform};
use clap::{command, Parser};
use log::info;

const DEFAULT_SERVER_PORT: u16 = 5000;

const LOG_LEVEL: &'static str = "info";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in local development mode (binds to 127.0.0.1)
    #[arg(long)]
    local: bool,
}

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    init_logs();

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
    info!("Initiating server...");

    let args = Args::parse();

    let bind_address = if args.local {
        "127.0.0.1"
    } else {
        "0.0.0.0"  // Default for production
    };

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_SERVER_PORT.to_string())
        .parse::<u16>()
        .unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .max_age(3600), // Cache preflight request for 1 hour
            )
            .service(web::scope("/api/v1")
                        .service(alive)
                        .service(transform)
                    )
    })
    .bind((bind_address, port))?
    .run();

    server.await?;

    Ok(())
}
