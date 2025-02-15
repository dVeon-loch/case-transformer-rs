use actix_web::{get, web, HttpResponse, Responder};
use log::debug;

use crate::transform::TransformRequest;

/// Exposed by the server to provide a simple "am I alive" endpoint
#[get("/alive")]
pub async fn alive() -> impl Responder {
    debug!("Hit /alive endpoint!");
    HttpResponse::Ok().body("Hello client!")
}

/// Exposed by the server to provide an endpoint for the case transforming function
#[get("/transform")]
pub async fn transform(payload: web::Json<TransformRequest>) -> impl Responder {
    debug!("Hit /transform endpoint!");
    HttpResponse::NotImplemented()
}

