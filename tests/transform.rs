//! Contains all general tests for the transform endpoint

#[cfg(test)]
mod tests {
    use actix_cors::Cors;
    use actix_web::{test, web, App};
    use case_transformer_rs::alive;

    use super::*;

    #[actix_web::test]
    async fn transform_upper_simple() {
        let app = test::init_service(App::new()
        .wrap(
            Cors::default()
                .allow_any_header()
                .allow_any_method()
                .allow_any_origin()
                .max_age(3600), // Cache preflight request for 1 hour
        )
        //.service(web::scope("/api/v1").service(transform)))
        ).await;
       
       todo!("Implement test and endpoint")
     }

    #[actix_web::test]
    async fn transform_lower_simple() {
        let app = test::init_service(App::new()
        .wrap(
            Cors::default()
                .allow_any_header()
                .allow_any_method()
                .allow_any_origin()
                .max_age(3600), // Cache preflight request for 1 hour
        )
        //.service(web::scope("/api/v1").service(transform)))
        ).await;
       
       todo!("Implement test and endpoint")
    }
}