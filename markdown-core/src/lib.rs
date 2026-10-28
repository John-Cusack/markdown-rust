//! `markdown-rust-core`: pure-Rust port of the Python-Markdown 3.11.2
//! render pipeline (preprocessors, block parser, inline engine,
//! treeprocessors, postprocessors, serializer, built-in extensions).
//!
//! No Python dependency. The whole document crosses the FFI boundary once
//! per `render` call (P1); per-call state (`toc`, `Meta`) is returned in
//! the response struct. Third-party extensions are NOT supported here —
//! the Python facade routes those renders to the vendored pure-Python
//! fallback, which is byte-identical upstream code by construction.

pub mod block;
pub mod consts;
pub mod context;
pub mod entities_gen;
pub mod entities_html5_gen;
pub mod entities_invalid_gen;
pub mod ext;
pub mod html;
pub mod inline;
pub mod model;
pub mod patterns_gen;
pub mod pipeline;
pub mod post;
pub mod pre;
pub mod ser;
pub mod tree;


/// Output format. Mirrors `Markdown.output_formats` (`core.py`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Html,
    Xhtml,
}
impl OutputFormat {
    pub fn parse(s: &str) -> Option<Self> {
        // Mirrors `set_output_format`: `fmt.lower().rstrip('145')`.
        let norm = s.to_lowercase();
        let norm = norm.trim_end_matches(['1', '4', '5']);
        match norm {
            "html" => Some(OutputFormat::Html),
            "xhtml" => Some(OutputFormat::Xhtml),
            _ => None,
        }
    }
}

/// Extension config scalar. Only JSON-like scalars cross the boundary;
/// callables (custom `slugify`, `build_url`, formatters) force the
/// Python fallback by construction (the facade never sends them).
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValue {
    Bool(bool),
    Int(i64),
    Str(String),
    StrList(Vec<String>),
    /// String-to-string mapping (e.g. the `abbr` `glossary`).
    StrMap(Vec<(String, String)>),
    None,
}

/// One requested extension: short name (as in `registerExtensions`) plus
/// its resolved configs.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionSpec {
    pub name: String,
    pub configs: Vec<(String, ConfigValue)>,
}
/// Everything the hot unit needs: the whole document in, no callbacks.
/// Queue entry: parent arena node + new children with ancestor chains.
pub type InsertQueueEntry = (usize, Vec<(usize, Vec<String>)>);
#[derive(Debug, Clone, PartialEq)]
pub struct RenderRequest {
    pub text: String,
    pub output_format: OutputFormat,
    pub tab_length: usize,
    pub extensions: Vec<ExtensionSpec>,
    /// Live `md.block_level_elements` (`None` = upstream defaults).
    pub block_level_elements: Option<Vec<String>>,
}

/// Table-of-contents token. Mirrors `toc.py` `toc_tokens` entries.
#[derive(Debug, Clone, PartialEq)]
pub struct TocToken {
    pub level: u32,
    pub id: String,
    pub name: String,
    pub html: String,
    pub data_toc_label: String,
    pub children: Vec<TocToken>,
}

/// Everything the hot unit returns: finished HTML out, plus the
/// per-document observable state the Python facade writes back
/// (`md.toc`, `md.toc_tokens`, `md.Meta`).
#[derive(Debug, Clone, PartialEq)]
pub struct RenderResponse {
    pub html: String,
    pub toc: String,
    pub toc_tokens: Vec<TocToken>,
    pub meta: Vec<(String, Vec<String>)>,
    /// Final `abbrs` map (live glossary base + document definitions),
    /// written back to the `AbbrExtension` instance like upstream.
    pub abbrs: Vec<(String, String)>,
}

/// The core refuses what it does not implement; the facade treats this
/// as a routing signal, never an error to the caller.
#[derive(Debug, Clone, PartialEq)]
pub enum RenderError {
    Unsupported(String),
}

pub type RenderResult = Result<RenderResponse, RenderError>;

/// Render one document through the hot unit (P1: whole document crosses
/// once). Anything unimplemented returns `Unsupported` for fallback
/// routing. `py_entry_depth`/`recursion_limit` feed the recursion model
/// (mirrors `nearing_recursion_limit`); the binding measures both live.
pub fn render(
    req: &RenderRequest,
    py_entry_depth: usize,
    recursion_limit: usize,
    footnote_unique_prefix: usize,
) -> RenderResult {
    pipeline::render_full(req, py_entry_depth, recursion_limit, footnote_unique_prefix)
}

/// Built-in extension short names this core build implements, in
/// upstream `registerExtensions` naming.
pub fn supported_extensions() -> Vec<String> {
    pipeline::implemented_extensions()
}

/// Crate version; tracks the upstream snapshot (`markdown.__version__`).
pub const UPSTREAM_VERSION: &str = "3.11.2";
