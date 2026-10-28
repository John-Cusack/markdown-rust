//! HTML serialization (`serializers.py`).
//!
//! Iterative stack machine mirroring `_serialize_html`. Only string tags
//! exist in our model (upstream Comment/PI/QName branches are dead for
//! pipeline-produced trees; the vendored pure-Python `serializers` module
//! stays the implementation for direct API calls).

use crate::consts::{HTML_EMPTY, RE_AMP};
use crate::model::Element;
use crate::OutputFormat;

fn escape_cdata(s: &str) -> String {
    let mut text = s.to_string();
    if text.contains('&') {
        text = RE_AMP.replace_all(&text, "&amp;").into_owned();
    }
    if text.contains('<') {
        text = text.replace('<', "&lt;");
    }
    if text.contains('>') {
        text = text.replace('>', "&gt;");
    }
    text
}

fn escape_attrib_html(s: &str) -> String {
    let mut text = s.to_string();
    if text.contains('&') {
        text = RE_AMP.replace_all(&text, "&amp;").into_owned();
    }
    if text.contains('<') {
        text = text.replace('<', "&lt;");
    }
    if text.contains('>') {
        text = text.replace('>', "&gt;");
    }
    if text.contains('"') {
        text = text.replace('"', "&quot;");
    }
    text
}

enum Item<'a> {
    Text(String),
    Node(&'a Element),
    Close(&'a str),
}

/// Serialize `root` (the `div`) to HTML.
pub fn serialize(root: &Element, format: OutputFormat) -> String {
    let mut out = String::new();
    let mut stack: Vec<Item> = vec![Item::Node(root)];
    while let Some(item) = stack.pop() {
        match item {
            Item::Text(s) => out.push_str(&s),
            Item::Close(tag) => {
                out.push_str("</");
                out.push_str(tag);
                out.push('>');
            }
            Item::Node(el) => {
                let tag = el.tag.as_str();
                out.push('<');
                out.push_str(tag);
                let mut attrs: Vec<(&str, &str)> = el
                    .attrib
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                attrs.sort();
                for (k, v) in attrs {
                    let esc = escape_attrib_html(v);
                    if k == esc && format == OutputFormat::Html {
                        // Boolean attribute in HTML mode.
                        out.push(' ');
                        out.push_str(&esc);
                    } else {
                        out.push(' ');
                        out.push_str(k);
                        out.push_str("=\"");
                        out.push_str(&esc);
                        out.push('"');
                    }
                }
                let lower = tag.to_lowercase();
                let is_empty_tag = HTML_EMPTY.contains(&lower.as_str());
                if format == OutputFormat::Xhtml && is_empty_tag {
                    out.push_str(" />");
                    if let Some(tail) = &el.tail {
                        out.push_str(&escape_cdata(tail.as_str()));
                    }
                    continue;
                }
                out.push('>');
                if let Some(text) = &el.text {
                    if lower == "script" || lower == "style" {
                        out.push_str(text.as_str());
                    } else {
                        out.push_str(&escape_cdata(text.as_str()));
                    }
                }
                // Stack is LIFO: push tail first so it emits after
                // children and the close tag.
                if let Some(tail) = &el.tail {
                    stack.push(Item::Text(escape_cdata(tail.as_str())));
                }
                if !is_empty_tag {
                    stack.push(Item::Close(tag));
                }
                for child in el.children.iter().rev() {
                    stack.push(Item::Node(child));
                }
            }
        }
    }
    out
}
