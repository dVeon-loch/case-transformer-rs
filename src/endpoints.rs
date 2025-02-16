use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use log::debug;

use crate::transform::{transform_case, TransformRequest};

/// Exposed by the server to provide a simple "am I alive" endpoint
#[get("/alive")]
pub async fn alive() -> impl Responder {
    debug!("Hit /alive endpoint!");
    HttpResponse::Ok().body("Hello client!")
}

/// Exposed by the server to provide an endpoint for the case transforming function
#[post("/transform")]
pub async fn transform(transform_request: web::Json<TransformRequest>) -> impl Responder {
    debug!("Hit /transform endpoint!");

    let transform_result = transform_case(transform_request.into_inner());

    match transform_result {
    Ok(result) => HttpResponse::Ok().content_type(ContentType::plaintext()).body(result),
    Err(err) => HttpResponse::InternalServerError().body(format!("Error transforming input request: {err}")),
    }
}

