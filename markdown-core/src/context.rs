//! Per-render state: the Rust mirror of the `Markdown` instance fields the
//! pipeline reads or writes.
//!
//! Anything the bridge fingerprints (`block_level_elements`,
//! `ESCAPED_CHARS`, `doc_tag`, `stripTopLevelTags`, registries) is
//! hardcoded to the upstream default here — a mutated instance never
//! reaches the core (the facade falls back instead).

use std::collections::HashMap;

use crate::model::Element;
use crate::{ConfigValue, ExtensionSpec, OutputFormat};

/// Raw-HTML stash entry. Core stages store strings; `md_in_html` stores
/// pre-parsed elements (same `rawHtmlBlocks: list[str|Element]` union).
#[derive(Debug, Clone)]
pub enum StashEntry {
    Html(String),
    Tree(Element),
}

/// Mirrors `Markdown` + `HtmlStash` for one render.
pub struct Context {
    pub tab_length: usize,
    pub output_format: OutputFormat,
    /// `md.references`: id (lowercased, stripped) -> (href, title?).
    pub references: HashMap<String, (String, Option<String>)>,
    /// `md.htmlStash.rawHtmlBlocks`.
    pub stash: Vec<StashEntry>,
    /// Extension specs in registration order (deduped last-wins by caller).
    pub extensions: Vec<ExtensionSpec>,
    /// `md.delimiters` token setup: token -> (tags, smart). Built from
    /// specs (`legacy_em` swaps the `em_strong` delimiter).
    pub delimiter_tokens: Vec<(String, bool)>,
    /// Recursion guard: Python frames at convert entry + per-level cost.
    /// Mirrors `util.nearing_recursion_limit()` (k=3 frames/level).
    pub py_entry_depth: usize,
    pub recursion_limit: usize,
    /// Block-nesting depth (parseChunk recursion).
    pub block_depth: usize,
    /// Set when the recursion budget refuses: unwind to the bridge
    /// fallback (which raises exactly like the oracle on pathological
    /// input, or renders identically on merely-deep input).
    pub declined: bool,
    /// `BlockParser.state`: one stack shared by the whole render.
    pub block_state: Vec<String>,
    /// Ordered block-processor chain for this render (built once in
    /// `parse_document` from specs; stable-sorted by priority).
    pub block_chain: Vec<crate::block::BlockKind>,
    /// Footnote state (fresh per render; mirrors `Extension.reset()`).
    /// Only `unique_prefix` carries across renders (read off the real
    /// extension instance by the facade).
    pub footnote_order: Vec<String>,
    pub footnotes: Vec<(String, String)>,
    pub found_refs: std::collections::HashMap<String, usize>,
    pub used_refs: std::collections::HashSet<String>,
    pub unique_prefix: usize,
    /// `abbr` definitions (per-render; seeded from the `glossary` config).
    pub abbrs: std::collections::HashMap<String, String>,
    /// Live `md.block_level_elements` (per-render from the facade; the
    /// `md_in_html` state machine reads it dynamically, including tags
    /// appended after construction).
    pub block_level_elements: Vec<String>,
    /// `TableProcessor` per-test state (`border`, `separator`).
    pub table_border: u8,
    pub table_separator: Vec<String>,
    pub toc: String,
    pub toc_tokens: Vec<crate::TocToken>,
    pub meta: Vec<(String, Vec<String>)>,
}

impl Context {
    pub fn new(
        tab_length: usize,
        output_format: OutputFormat,
        extensions: Vec<ExtensionSpec>,
        py_entry_depth: usize,
        recursion_limit: usize,
        block_level_elements: Option<Vec<String>>,
    ) -> Self {
        let delimiter_tokens = if extensions.iter().any(|e| e.name == "legacy_em") {
                // `legacy_em` replaces the `_` delimiter with a non-smart one.
                vec![("*".to_string(), false), ("_".to_string(), false)]
            } else {
                vec![("*".to_string(), false), ("_".to_string(), true)]
            };
        Context {
            tab_length,
            output_format,
            references: HashMap::new(),
            stash: Vec::new(),
            extensions,
            delimiter_tokens,
            py_entry_depth,
            recursion_limit,
            block_depth: 0,
            declined: false,
            block_state: Vec::new(),
            block_chain: Vec::new(),
            footnote_order: Vec::new(),
            footnotes: Vec::new(),
            found_refs: std::collections::HashMap::new(),
            used_refs: std::collections::HashSet::new(),
            unique_prefix: 0,
            abbrs: std::collections::HashMap::new(),
            block_level_elements: block_level_elements.unwrap_or_else(|| {
                crate::consts::BLOCK_LEVEL_ELEMENTS.iter().map(|s| s.to_string()).collect()
            }),
            table_border: 0,
            table_separator: Vec::new(),
            toc: String::new(),
            toc_tokens: Vec::new(),
            meta: Vec::new(),
        }
    }

    pub fn bst_set(&mut self, s: &str) {
        self.block_state.push(s.to_string());
    }
    pub fn bst_reset(&mut self) {
        self.block_state.pop();
    }
    /// Append a `Meta` value, preserving first-occurrence key order.
    pub fn meta_append(&mut self, key: String, value: String) {
        match self.meta.iter_mut().find(|(k, _)| *k == key) {
            Some((_, vs)) => vs.push(value),
            None => self.meta.push((key, vec![value])),
        }
    }
    pub fn bst_is(&self, s: &str) -> bool {
        self.block_state.last().map(|t| t == s).unwrap_or(false)
    }

    pub fn has_extension(&self, name: &str) -> bool {
        self.extensions.iter().any(|e| e.name == name)
    }

    /// `Markdown.ESCAPED_CHARS` (+ smarty's `"`/`'` when present).
    pub fn escaped_chars(&self) -> Vec<char> {
        let mut v = vec![
            '\\', '`', '*', '_', '{', '}', '[', ']', '(', ')', '>', '#', '+', '-', '.', '!',
        ];
        if self.has_extension("smarty") {
            v.push('"');
            v.push('\'');
        }
        if self.has_extension("tables") {
            v.push('|');
        }
        v
    }

    pub fn ext_config(&self, ext: &str, key: &str) -> Option<&ConfigValue> {
        self.extensions.iter().rev().find_map(|e| {
            if e.name == ext {
                e.configs.iter().rev().find(|(k, _)| k == key).map(|(_, v)| v)
            } else {
                None
            }
        })
    }

    pub fn ext_bool(&self, ext: &str, key: &str, default: bool) -> bool {
        match self.ext_config(ext, key) {
            Some(ConfigValue::Bool(b)) => *b,
            Some(ConfigValue::Int(i)) => *i != 0,
            Some(ConfigValue::Str(s)) => !s.is_empty(),
            _ => default,
        }
    }

    pub fn ext_str(&self, ext: &str, key: &str, default: &str) -> String {
        match self.ext_config(ext, key) {
            Some(ConfigValue::Str(s)) => s.clone(),
            _ => default.to_string(),
        }
    }

    /// `Markdown.is_block_level`: strings only (our tags always are).
    pub fn is_block_level(&self, tag: &str) -> bool {
        let t = tag.to_lowercase();
        let t = t.trim_end_matches('/');
        self.block_level_elements.iter().any(|e| e == t)
    }

    /// `HtmlStash.store`: append + return `HTML_PLACEHOLDER % key`.
    pub fn stash_store(&mut self, entry: StashEntry) -> String {
        let key = self.stash.len();
        self.stash.push(entry);
        crate::consts::placeholder_html(key)
    }

    pub fn stash_html_count(&self) -> usize {
        self.stash.len()
    }

    /// Blockquote nesting guard. `true` when the `blockquote` processor
    /// may run at this depth (mirrors `nearing_recursion_limit()`).
    pub fn blockquote_allowed(&self) -> bool {
        // Python frames at the guard: entry + 5 (convert entry -> test) +
        // 3 per nesting level (parseBlocks/run/parseChunk). Calibrated:
        // direct call trips at 299 levels under limit 1000.
        let depth = self.py_entry_depth + 5 + 3 * self.block_depth;
        self.recursion_limit.saturating_sub(depth) >= 100
    }
}
