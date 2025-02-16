use kuchikiki::traits::TendrilSink;
use markup5ever::{local_name, ns, QualName};
use markup5ever::namespace_url;
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
    let context_name = QualName::new(None, ns!(),  local_name!("div"));

    let mut html = kuchikiki::parse_fragment(context_name, vec![]).from_utf8().one(transform_request.html.as_bytes());

    transform_text_nodes(&html, &transform_request.transform);

    Ok(html.to_string())
}

fn transform_text_nodes(node: &kuchikiki::NodeRef, transform_method: &TransformMethod) {
    if let Some(text) = node.as_text() {
        let mut contents = text.borrow_mut();
        *contents = match transform_method {
            TransformMethod::UPPER => contents.to_uppercase(),
            TransformMethod::LOWER => contents.to_lowercase(),
        };
    } else {
        for child in node.children() {
            transform_text_nodes(&child, transform_method);
        }
    }
}