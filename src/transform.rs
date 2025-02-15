use eyre::bail;
use serde::{Deserialize, Serialize};
use transform_method::TransformMethod;

pub mod transform_method;

#[derive(Deserialize, Serialize)]
pub struct TransformRequest {
    transform: TransformMethod,
    html: String,
}

impl TransformRequest {
    pub fn new(transform_method: TransformMethod, html: String) -> Self {
        Self {
            transform: transform_method,
            html,
        }
    }
}

pub fn transform_case(transform_request: TransformRequest) -> eyre::Result<String> {
    bail!("transform logic not implemented yet")
}