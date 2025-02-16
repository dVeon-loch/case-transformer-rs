use kuchikiki::traits::TendrilSink;
use markup5ever::namespace_url;
use markup5ever::{local_name, ns, QualName};
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

/// Transforms the text nodes of a given html string that may or may not be a fragment, into
/// either uppercase or lowercase depending on the `TransformMethod`
pub fn transform_case(transform_request: TransformRequest) -> eyre::Result<String> {
    let context_name = QualName::new(None, ns!(), local_name!("div"));

    let has_html_tag =
        transform_request.html.contains("<html>") && transform_request.html.contains("</html>");

    let html = kuchikiki::parse_fragment(context_name, vec![])
        .from_utf8()
        .one(transform_request.html.as_bytes());

    transform_text_nodes(&html, &transform_request.transform);

    let output_string = match has_html_tag {
        false => html
            .to_string()
            .replace("<html>", "")
            .replace("</html>", ""),
        true => html.to_string(),
    };

    Ok(output_string)
}

/// Recursively transforms the nodes of an html fragment or full document and transforms
/// their textual content to/from uppercase/lowercase depending on the `TransformMethod``
fn transform_text_nodes(node: &kuchikiki::NodeRef, transform_method: &TransformMethod) {
    if let Some(text) = node.as_text() {
        if is_inside_p(node) {
            let mut contents = text.borrow_mut();
            *contents = match transform_method {
                TransformMethod::UPPER => contents.to_uppercase(),
                TransformMethod::LOWER => contents.to_lowercase(),
            };
        }
    } else {
        for child in node.children() {
            transform_text_nodes(&child, transform_method);
        }
    }
}

fn is_inside_p(node: &kuchikiki::NodeRef) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if let Some(element) = parent.as_element() {
            if element.name.local == local_name!("p") {
                return true;
            }
        }
        current = parent.parent();
    }
    false
}
