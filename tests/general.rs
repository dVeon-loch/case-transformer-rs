//! Contains general tests for the webserver

#[cfg(test)]
mod tests {
    use actix_cors::Cors;
    use actix_web::{test, web, App};
    use case_transformer_rs::alive;

    use super::*;

    #[actix_web::test]
    async fn test_alive() {
        let app = test::init_service(App::new()
        .wrap(
            Cors::default()
                .allow_any_header()
                .allow_any_method()
                .allow_any_origin()
                .max_age(3600), // Cache preflight request for 1 hour
        )
        .service(web::scope("/api/v1").service(alive))).await;
        let req = test::TestRequest::get().uri("/api/v1/alive")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}