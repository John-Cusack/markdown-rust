//! Minimal Element tree mirroring the `xml.etree.ElementTree` subset the
//! pipeline produces and consumes.
//!
//! Differences from `ElementTree`, all load-bearing for byte identity:
//! - `tag` is always a Rust `String`. Upstream trees can carry non-string
//!   tags (Comment/PI/QName), for which `is_block_level` is always false;
//!   the render pipeline never *produces* such nodes from Markdown source
//!   (raw HTML round-trips as stashed strings), so the model excludes them.
//!   Direct serializer-level tests (`test_apis.py` ETree cases) exercise
//!   the vendored pure-Python `serializers` module, not this model.
//! - Attributes are an insertion-ordered `Vec`; the serializer sorts them
//!   lexically, exactly like `items.sort()` in `serializers.py`.
//! - Text/tail carry the `AtomicString` bit explicitly (`util.py:186`).
//!   The inline engine must preserve it: atomic text is never re-scanned
//!   for patterns, and placeholder expansion re-wraps remainders as atomic
//!   when the input was atomic (`treeprocessors.py:245-247`).

/// `str` vs [`AtomicString`](https://python-markdown.github.io):
/// atomic text is opaque to the inline engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Text {
    Normal(String),
    Atomic(String),
}

impl Text {
    pub fn as_str(&self) -> &str {
        match self {
            Text::Normal(s) | Text::Atomic(s) => s,
        }
    }

    pub fn is_atomic(&self) -> bool {
        matches!(self, Text::Atomic(_))
    }

    pub fn into_string(self) -> String {
        match self {
            Text::Normal(s) | Text::Atomic(s) => s,
        }
    }
}

/// One element node. `text`/`tail` mirror `Element.text`/`Element.tail`
/// (`None` == upstream `None`, never `""` unless explicitly assigned).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub tag: String,
    pub attrib: Vec<(String, String)>,
    pub text: Option<Text>,
    pub tail: Option<Text>,
    pub children: Vec<Element>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Element {
            tag: tag.to_string(),
            attrib: Vec::new(),
            text: None,
            tail: None,
            children: Vec::new(),
        }
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrib
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    pub fn set_attr(&mut self, key: &str, value: &str) {
        if let Some(slot) = self.attrib.iter_mut().find(|(k, _)| k == key) {
            slot.1 = value.to_string();
        } else {
            self.attrib.push((key.to_string(), value.to_string()));
        }
    }

    pub fn append_child(&mut self, child: Element) {
        self.children.push(child);
    }

    pub fn insert_child(&mut self, index: usize, child: Element) {
        let idx = index.min(self.children.len());
        self.children.insert(idx, child);
    }

    /// `Element.itertext()`: text + descendants, in document order.
    pub fn itertext(&self) -> String {
        let mut out = String::new();
        if let Some(t) = &self.text {
            out.push_str(t.as_str());
        }
        for child in &self.children {
            out.push_str(&child.itertext());
            if let Some(t) = &child.tail {
                out.push_str(t.as_str());
            }
        }
        out
    }
}
