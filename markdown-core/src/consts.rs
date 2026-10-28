//! Shared constants and helpers mirroring `markdown/util.py` +
//! `markdown/serializers.py` module level.
//!
//! Regex sources come from `patterns_gen.rs` (machine-dumped from
//! upstream; fidelity covered by `tests/patterns_diff.rs`).

use std::sync::LazyLock;

use fancy_regex::Regex as F;

pub const STX: char = '\u{2}';
pub const ETX: char = '\u{3}';

pub const INLINE_PLACEHOLDER_PREFIX: &str = "\u{2}klzzwxh:";
pub const AMP_SUBSTITUTE: &str = "\u{2}amp\u{3}";

/// `INLINE_PLACEHOLDER % id` with `id` zero-padded to 4 (`"%04d"`).
pub fn placeholder_inline(id: usize) -> String {
    format!("{INLINE_PLACEHOLDER_PREFIX}{id:04}{ETX}")
}

/// `HTML_PLACEHOLDER % key`.
pub fn placeholder_html(key: usize) -> String {
    format!("\u{2}wzxhzdk:{key}{ETX}")
}

/// `TAG_PLACEHOLDER % key` (md_in_html).
pub fn placeholder_tag(key: usize) -> String {
    format!("\u{2}hzzhzkh:{key}{ETX}")
}

pub fn parse_inline_placeholder(s: &str) -> Option<(usize, usize)> {
    let m = RE_INLINE_PH.find(s).ok()??;
    let id: usize = m.as_str()[INLINE_PLACEHOLDER_PREFIX.len() + 1..m.as_str().len() - 1]
        .parse()
        .ok()?;
    Some((id, m.end()))
}

/// `util.BLOCK_LEVEL_ELEMENTS`, order-preserved copy.
pub static BLOCK_LEVEL_ELEMENTS: &[&str] = &[
    "address", "article", "aside", "blockquote", "details", "div", "dl", "fieldset", "figcaption",
    "figure", "footer", "form", "h1", "h2", "h3", "h4", "h5", "h6", "header", "hgroup", "hr",
    "main", "menu", "nav", "ol", "p", "pre", "section", "table", "ul", "canvas", "colgroup",
    "dd", "body", "dt", "group", "html", "iframe", "li", "legend", "math", "map", "noscript",
    "output", "object", "option", "progress", "script", "style", "summary", "tbody", "td",
    "textarea", "tfoot", "th", "thead", "tr", "video", "center",
];

/// `ElementTree.HTML_EMPTY` (lowercased compare in the serializer).
pub static HTML_EMPTY: &[&str] = &[
    "area", "base", "basefont", "br", "col", "embed", "frame", "hr", "img", "input", "isindex",
    "link", "meta", "param", "source", "track", "wbr",
];

/// `htmlparser.void_tags`.
pub static VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// CDATA content elements (`HTMLParser.CDATA_CONTENT_ELEMENTS`, py3.12).
pub static CDATA_ELEMENTS: &[&str] = &["script", "style", "xmp", "iframe", "noembed", "noframes"];

/// `util.code_escape`: `&` then `<` then `>` (order matters).
pub fn code_escape(s: &str) -> String {
    let mut out = s.to_string();
    if out.contains('&') {
        out = out.replace('&', "&amp;");
    }
    if out.contains('<') {
        out = out.replace('<', "&lt;");
    }
    if out.contains('>') {
        out = out.replace('>', "&gt;");
    }
    out
}

/// Compile a dumped upstream pattern (panics on failure: the dump is
/// machine-verified by `tests/patterns_diff.rs`).
///
/// Results are cached by pattern string (P6: compile once): inline
/// matching builds some patterns per call site (delimiter boundaries,
/// per-link regexes), and `fancy_regex` compilation dominates a render.
/// The cache is bounded (a few hundred distinct dumped patterns) and
/// entries are immutable.
pub fn compile(pat: &str) -> F {
    use std::collections::HashMap;
    use std::sync::{LazyLock, RwLock};
    static CACHE: LazyLock<RwLock<HashMap<String, F>>> =
        LazyLock::new(|| RwLock::new(HashMap::new()));
    if let Some(re) = CACHE.read().unwrap().get(pat) {
        return re.clone();
    }
    let re = F::new(pat).expect("dumped upstream pattern must compile");
    CACHE.write().unwrap().insert(pat.to_string(), re.clone());
    re
}

macro_rules! rx {
    ($name:ident, $src:expr) => {
        pub static $name: LazyLock<F> = LazyLock::new(|| compile($src));
    };
}
rx!(RE_AMP, crate::patterns_gen::RE_SER_RE_AMP);
rx!(RE_INLINE_PH, crate::patterns_gen::RE_UTIL_INLINE_PLACEHOLDER_RE);
rx!(RE_HTML_PH, crate::patterns_gen::RE_UTIL_HTML_PLACEHOLDER_RE);
rx!(RE_UNESCAPE, crate::patterns_gen::RE_TP_UNESCAPETREEPROCESSOR_RE);
