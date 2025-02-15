use actix_web::{get, HttpResponse, Responder};
use log::debug;

/// Exposed by the server to provide a simple "am I alive" endpoint
#[get("/alive")]
pub async fn alive() -> impl Responder {
    debug!("Hit /alive endpoint!");
    HttpResponse::Ok().body("Hello client!")
}
