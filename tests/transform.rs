//! Contains all general tests for the transform endpoint

#[cfg(test)]
mod tests {
    use actix_cors::Cors;
    use actix_web::{test, web, App};
    use case_transformer_rs::{endpoints::transform, transform::{transform_method::TransformMethod, TransformRequest}};

    use super::*;

    #[actix_web::test]
    async fn transform_upper_simple() {
        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .allow_any_header()
                        .allow_any_method()
                        .allow_any_origin()
                        .max_age(3600), // Cache preflight request for 1 hour
                )
                .service(transform),
        )
        .await;

        let request_payload = TransformRequest::new(TransformMethod::LOWER, r#"<p>Hello world</p>"#.to_string());

        let req = test::TestRequest::get()
            .uri("/transform")
            .set_json(request_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let content_type = resp.headers().get("Content-Type").unwrap();
        assert_eq!(content_type, "text/plain");
        
        let body = test::read_body(resp).await;

        let body_str = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body_str, r#"<p>HELLO WORLD</p>"#);
    }

    #[actix_web::test]
    async fn transform_lower_simple() {
        let app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .allow_any_header()
                        .allow_any_method()
                        .allow_any_origin()
                        .max_age(3600), // Cache preflight request for 1 hour
                )
                .service(transform),
        )
        .await;

        let request_payload = TransformRequest::new(TransformMethod::LOWER, r#"<p>Hello WORLD</p>"#.to_string());

        let req = test::TestRequest::get()
            .uri("/transform")
            .set_json(request_payload)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        
        let content_type = resp.headers().get("Content-Type").unwrap();
        assert_eq!(content_type, "text/plain");

        let body = test::read_body(resp).await;

        let body_str = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body_str, r#"<p>hello world</p>"#);
    }
}
