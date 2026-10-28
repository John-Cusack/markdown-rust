//! `HTMLExtractor` (`htmlparser.py`): block-level HTML detection.
//!
//! A single-pass tokenizer replicating the stdlib `HTMLParser.goahead`
//! driver (with `end=1`) plus every monkeypatched regex and every
//! `HTMLExtractor` handler override. Semantics pinned to CPython 3.12's
//! `html.parser` (the version `super()` calls resolve to); `<!`-edge
//! behavior is inherently version-coupled upstream, noted in the ADR.
//!
//! Positions are byte indices into the source; line/column queries count
//! chars exactly like `updatepos` (`lineno` 1-based, `offset` in chars).

use fancy_regex::Regex as F;

use crate::consts::{CDATA_ELEMENTS, VOID_TAGS};
use crate::context::{Context, StashEntry};
use crate::model::{Element, Text};

/// Byte offsets of every line start, plus the char count twin.
struct Lines {
    byte_starts: Vec<usize>,
}

impl Lines {
    fn new(text: &str) -> Self {
        let mut byte_starts = vec![0];
        for (b, ch) in text.char_indices() {
            if ch == '\n' {
                byte_starts.push(b + 1);
            }
        }
        Lines { byte_starts }
    }

    fn line_of(&self, byte_pos: usize) -> usize {
        self.byte_starts.partition_point(|&s| s <= byte_pos) - 1
    }

    fn line_start_b(&self, line: usize) -> usize {
        self.byte_starts[line]
    }

    fn col_chars(&self, text: &str, byte_pos: usize) -> usize {
        text[self.line_start_b(self.line_of(byte_pos))..byte_pos].chars().count()
    }
}

pub struct Extractor<'a, 'c> {
    text: &'a str,
    lines: Lines,
    ctx: &'c mut Context,
    pos: usize,
    cdata: Option<String>,
    starttag_text: Option<String>,
    inraw: bool,
    intail: bool,
    stack: Vec<String>,
    inline_stack: Vec<String>,
    cache: Vec<String>,
    cleandoc: Vec<String>,
    /// `md_in_html` mode (`HTMLExtractorExtra`).
    extra: bool,
    /// Open markdown-tag stack (`mdstack`), parallel state/started flags.
    mdstack: Vec<String>,
    mdstate: Vec<Option<u8>>,
    mdstarted: Vec<bool>,
    /// `treebuilder` equivalent: open elements under construction.
    tstack: Vec<Element>,
}

pub fn extract_html(ctx: &mut Context, source: &str) -> Vec<String> {
    let extra = ctx.has_extension("md_in_html");
    let mut ex = Extractor {
        text: source,
        lines: Lines::new(source),
        ctx,
        pos: 0,
        cdata: None,
        starttag_text: None,
        inraw: false,
        intail: false,
        stack: Vec::new(),
        inline_stack: Vec::new(),
        cache: Vec::new(),
        cleandoc: Vec::new(),
        extra,
        mdstack: Vec::new(),
        mdstate: Vec::new(),
        mdstarted: Vec::new(),
        tstack: Vec::new(),
    };
    ex.run();
    // `HTMLExtractorExtra.close`: unclosed markdown tags.
    ex.close_extra();
    // `HTMLExtractor.close()` tail: flush an unclosed raw block / cached
    // tail (e.g. an `intail` stash). The `rawdata` remainder itself is
    // always empty after single-pass `end=1` tokenizing.
    if !ex.cache.is_empty() {
        let raw = ex.cache.join("");
        let key = ex.ctx.stash.len();
        ex.ctx.stash.push(StashEntry::Html(raw));
        ex.cleandoc.push(crate::consts::placeholder_html(key));
    }
    std::mem::take(&mut ex.cleandoc)
}
/// `to_string` on slices passed to `&mut self` methods is borrowck-required
/// (the slice borrows `self`); not elidable.
#[allow(clippy::unnecessary_to_owned)]
impl<'a, 'c> Extractor<'a, 'c> {
    fn n(&self) -> usize {
        self.text.len()
    }

    fn at(&self, s: &str) -> bool {
        self.text[self.pos..].starts_with(s)
    }

    fn line_offset_b(&self, pos: usize) -> usize {
        let line = self.lines.line_of(pos);
        self.lines.line_start_b(line)
    }

    /// `at_line_start`: up to 3 chars of whitespace before `pos` on its line.
    fn at_line_start(&self, pos: usize) -> bool {
        if pos == 0 {
            return true;
        }
        let col = self.lines.col_chars(self.text, pos);
        if col > 3 {
            return false;
        }
        self.text[self.line_offset_b(pos)..pos].trim().is_empty()
    }

    fn run(&mut self) {
        // `interesting` in normal mode.
        let interesting = crate::consts::compile("[&<]");
        while self.pos < self.n() {
            let j = if let Some(tag) = self.cdata.clone() {
                let pat = format!("(?i)</\\s*{tag}\\s*>");
                let re = crate::consts::compile(&pat);
                re.find_from_pos(self.text, self.pos).ok().flatten().map(|m| m.start())
            } else {
                interesting.find_from_pos(self.text, self.pos).ok().flatten().map(|m| m.start())
            };
            match j {
                Some(j) if j > self.pos => {
                    let data = self.text[self.pos..j].to_string();
                    self.handle_data(&data);
                    self.pos = j;
                }
                None => {
                    let data = self.text[self.pos..].to_string();
                    self.handle_data(&data);
                    self.pos = self.n();
                    break;
                }
                _ => {}
            }
            if self.pos >= self.n() {
                break;
            }
            if self.at("<") {
                let k = if starttagopen_at(self.text, self.pos) {
                    self.parse_starttag()
                } else if self.at("</") {
                    self.parse_endtag()
                } else if self.at("<!--") {
                    self.parse_comment()
                } else if self.at("<?") {
                    self.parse_pi()
                } else if self.at("<!") {
                    self.parse_html_declaration()
                } else {
                    // `(i+1) < n or end` with `end=1`: always true.
                    self.handle_data("<");
                    (self.pos + 1) as i64
                };
                // `k < 0` end-of-input forcing (goahead `end=1` branch).
                self.pos = if k < 0 { self.end_behavior() } else { k as usize };
            } else if self.at("&#") {
                self.parse_charref();
            } else if self.at("&") {
                self.parse_entityref();
            } else {
                // `interesting` lied (unreachable): consume one char.
                let ch = self.text[self.pos..].chars().next().unwrap();
                let data = ch.to_string();
                self.handle_data(&data);
                self.pos += ch.len_utf8();
            }
        }
    }

    /// The `k < 0` branch of `goahead` with `end=1`.
    fn end_behavior(&mut self) -> usize {
        let pos = self.pos;
        if starttagopen_at(self.text, pos) {
            // pass
        } else if self.text[pos..].starts_with("</") {
            if pos + 2 == self.n() {
                self.handle_data("</");
            } else if endtagopen_at(self.text, pos) {
                // pass
            } else {
                let data = self.text[pos + 2..].to_string();
                self.handle_comment(&data);
            }
        } else if self.text[pos..].starts_with("<!--") {
            let mut j = self.n();
            for suffix in ["--!", "--", "-"] {
                if self.text[pos + 4..].ends_with(suffix) {
                    j -= suffix.len();
                    break;
                }
            }
            let data = self.text[pos + 4..j].to_string();
            self.handle_comment(&data);
        } else if self.text[pos..].starts_with("<![CDATA[") {
            let data = self.text[pos + 3..].to_string();
            self.unknown_decl(&data);
        } else if self.text[pos..].to_lowercase().starts_with("<!doctype") {
            let data = self.text[pos + 2..].to_string();
            self.handle_decl(&data);
        } else if self.text[pos..].starts_with("<!") {
            let data = self.text[pos + 2..].to_string();
            self.handle_comment(&data);
        } else if self.text[pos..].starts_with("<?") {
            let data = self.text[pos + 2..].to_string();
            self.handle_pi(&data);
        }
        self.n()
    }
    // ---- token parsers (return byte index to continue at, or -1) ----

    fn parse_starttag(&mut self) -> i64 {
        let pos = self.pos;
        // Treat `</>` as normal data.
        if self.text[pos..].starts_with("</>") {
            self.handle_data("</>");
            return (pos + 3) as i64;
        }
        let endpos = check_whole_start_tag(self.text, pos);
        if endpos < 0 {
            let e = char_end(self.text, pos + 1);
            self.handle_data(&self.text[pos..e].to_string());
            return e as i64;
        }
        let endpos = endpos as usize;
        // Parse tag name + attrs (attrs discarded downstream, but `k`
        // must land exactly where the stdlib loop leaves it).
        let mut k = tagfind_end(self.text, pos + 1);
        let tag = tag_name_at(self.text, pos + 1).to_lowercase();
        loop {
            if k >= endpos {
                break;
            }
            let m = match ATTRFIND.find_from_pos(self.text, k) {
                Ok(m) => m,
                Err(_) => break,
            };
            let m = match m {
                Some(m) if m.start() == k => m,
                _ => break,
            };
            k = m.end();
        }
        let end = self.text[k..endpos].trim();
        if end != ">" && end != "/>" {
            self.handle_data(&self.text[pos..endpos].to_string());
            return endpos as i64;
        }
        self.starttag_text = Some(self.text[pos..endpos].to_string());
        if self.extra {
            let k = tagfind_end(self.text, pos + 1);
            let attrs = self.parse_attrs_at(k, endpos);
            if end.ends_with("/>") {
                self.extra_handle_startendtag(&tag, &attrs);
            } else {
                if CDATA_ELEMENTS.contains(&tag.as_str()) {
                    self.cdata = Some(tag.clone());
                }
                self.extra_handle_starttag(&tag, &attrs);
            }
        } else if end.ends_with("/>") {
            self.base_handle_startendtag(&tag);
        } else {
            if CDATA_ELEMENTS.contains(&tag.as_str()) {
                self.cdata = Some(tag.clone());
            }
            self.base_handle_starttag(&tag);
        }
        endpos as i64
    }

    fn parse_endtag(&mut self) -> i64 {
        let pos = self.pos;
        // `_HTMLParser.parse_endtag` override.
        let s3 = &self.text[pos..char_end(self.text, pos + 3).min(self.n())];
        let is_data = s3.len() < 3
            || !s3.chars().last().map(|c| c.is_ascii_alphabetic()).unwrap_or(false);
        if is_data {
            let e = char_end(self.text, pos + 2).min(self.n());
            self.handle_data(&self.text[pos..e].to_string());
            return e as i64;
        }
        // super().parse_endtag
        if self.text[pos + 2..].find('>').is_none() {
            self.handle_data(&self.text[pos..char_end(self.text, pos + 2).min(self.n())].to_string());
            return char_end(self.text, pos + 2).min(self.n()) as i64;
        }
        if !endtagopen_at(self.text, pos) {
            if self.text[pos + 2..].starts_with('>') {
                return (pos + 3) as i64; // `</>` ignored (unreachable: starttag branch)
            }
            return self.parse_bogus_comment();
        }
        let m = LOCATETAGEND.find(&self.text[pos + 2..]).unwrap().unwrap();
        let j = pos + 2 + m.end();
        if !self.text[..j].ends_with('>') {
            self.handle_data(&self.text[pos..char_end(self.text, pos + 2).min(self.n())].to_string());
            return char_end(self.text, pos + 2).min(self.n()) as i64;
        }
        let tag = tag_name_at(self.text, pos + 2).to_lowercase();
        self.handle_endtag(&tag);
        self.cdata = None;
        j as i64
    }
    fn parse_comment(&mut self) -> i64 {
        let pos = self.pos;
        match COMMENTCLOSE.find(&self.text[pos + 4..]) {
            Ok(Some(m)) => {
                let j = pos + 4 + m.start();
                let data = self.text[pos + 4..j].to_string();
                self.handle_comment(&data);
                (pos + 4 + m.end()) as i64
            }
            _ => {
                self.handle_data("<");
                (pos + 1) as i64
            }
        }
    }

    fn parse_html_declaration(&mut self) -> i64 {
        let pos = self.pos;
        // Extra: `or self.mdstack` also takes the tag path.
        let in_extra = self.extra && !self.mdstack.is_empty();
        if self.at_line_start(pos) || self.intail || in_extra {
            if self.text[pos..].starts_with("<![")
                && !self.text[pos..].starts_with("<![CDATA[")
            {
                // Python <3.13 workaround (#1534): bogus comment, unaltered.
                let r = self.parse_bogus_comment();
                if r < 0 {
                    let e = char_end(self.text, pos + 1);
                    self.handle_data(&self.text[pos..e].to_string());
                    return e as i64;
                }
                return r;
            }
            return self.super_declaration();
        }
        // Not the beginning of a raw block: plain data.
        self.handle_data("<!");
        (pos + 2) as i64
    }

    /// stdlib `parse_html_declaration` (3.12 semantics).
    fn super_declaration(&mut self) -> i64 {
        let pos = self.pos;
        if self.text[pos..].starts_with("<!--[") {
            return self.parse_comment();
        }
        if self.text[pos..].starts_with("<![CDATA[") {
            match self.text[pos + 9..].find("]]>") {
                Some(k) => {
                    let data = self.text[pos + 3..pos + 9 + k].to_string();
                    self.unknown_decl(&data);
                    (pos + 9 + k + 3) as i64
                }
                None => -1,
            }
        } else if self.text[pos..].to_lowercase().starts_with("<!doctype") {
            match self.text[pos + 9..].find('>') {
                Some(k) => {
                    let data = self.text[pos + 2..pos + 9 + k].to_string();
                    self.handle_decl(&data);
                    (pos + 9 + k + 1) as i64
                }
                None => -1,
            }
        } else if self.text[pos..].starts_with("<![") {
            match self.text[pos + 3..].find('>') {
                Some(k) => {
                    let gt = pos + 3 + k;
                    if self.text[..gt].ends_with(']') {
                        let data = self.text[pos + 3..gt - 1].to_string();
                        self.unknown_decl(&data);
                    } else {
                        let data = self.text[pos + 2..gt].to_string();
                        self.handle_comment(&data);
                    }
                    (gt + 1) as i64
                }
                None => -1,
            }
        } else {
            // Stdlib dispatches remaining `<!...` to `self.parse_bogus_comment`
            // (overridden: raw, report=0) — never the commenting super path.
            self.parse_bogus_comment()
        }
    }

    /// `HTMLExtractor.parse_bogus_comment(i, report=0)`.
    fn parse_bogus_comment(&mut self) -> i64 {
        let pos = self.pos;
        match self.text[pos + 2..].find('>') {
            Some(k) => {
                let end = pos + 2 + k + 1;
                let data = self.text[pos..end].to_string();
                self.handle_empty_tag(&data, false);
                end as i64
            }
            None => -1,
        }
    }

    fn parse_pi(&mut self) -> i64 {
        let pos = self.pos;
        // Extra: `or self.mdstack` also takes the tag path.
        let in_extra = self.extra && !self.mdstack.is_empty();
        if !(self.at_line_start(pos) || self.intail || in_extra) {
            // Not the beginning of a raw block: plain data.
            self.handle_data("<?");
            return (pos + 2) as i64;
        }
        match PICLOSE.find(&self.text[pos + 2..]) {
            Ok(Some(m)) => {
                let j = pos + 2 + m.start();
                let data = self.text[pos + 2..j].to_string();
                self.handle_pi(&data);
                (pos + 2 + m.end()) as i64
            }
            _ => -1,
        }
    }

    fn parse_charref(&mut self) {
        let pos = self.pos;
        match CHARREF.find(&self.text[pos..]) {
            Ok(Some(m)) if m.start() == 0 => {
                let name = self.text[pos + 2..pos + m.end() - 1].to_string();
                self.handle_charref(&name);
                let mut k = pos + m.end();
                if !self.text[k - 1..].starts_with(';') {
                    k -= 1;
                }
                self.pos = k;
            }
            _ => {
                if self.text[pos..].contains(';') {
                    let e = char_end(self.text, pos + 2);
                    self.handle_data(&self.text[pos..e].to_string());
                    self.pos = e;
                } else {
                    // Wait for more data (feed-time `break`): with single
                    // `feed` + `close` this is the tail; emit it raw.
                    let data = self.text[pos..].to_string();
                    self.handle_data(&data);
                    self.pos = self.n();
                }
            }
        }
    }

    fn parse_entityref(&mut self) {
        let pos = self.pos;
        // `entityref` and (patched) `incomplete` are the same object, so
        // the incomplete branch is dead: only the match / `&` cases remain.
        match ENTITYREF.find(&self.text[pos..]) {
            Ok(Some(m)) if m.start() == 0 => {
                let name = m.as_str()[1..m.as_str().len()].to_string();
                let name = name.strip_suffix(';').unwrap_or(&name).to_string();
                self.handle_entityref(&name);
                let mut k = pos + m.end();
                if !self.text[k - 1..].starts_with(';') {
                    k -= 1;
                }
                self.pos = k;
            }
            _ => {
                if pos + 1 < self.n() {
                    self.handle_data("&");
                    self.pos = pos + 1;
                } else {
                    self.pos = self.n();
                }
            }
        }
    }
    // ---- handlers (HTMLExtractor methods) ----

    fn store(&mut self, html: String) -> String {
        let key = self.ctx.stash.len();
        self.ctx.stash.push(StashEntry::Html(html));
        crate::consts::placeholder_html(key)
    }

    fn base_handle_starttag(&mut self, tag: &str) {
        if tag == "hr" {
            let t = self.starttag_text.clone().unwrap();
            self.handle_empty_tag(&t, self.ctx.is_block_level(tag));
            return;
        }
        if self.ctx.is_block_level(tag)
            && (self.intail || (self.at_line_start(self.pos) && !self.inraw))
        {
            self.inraw = true;
            self.cleandoc.push("\n".to_string());
        }
        let text = self.starttag_text.clone().unwrap();
        if self.inraw {
            self.stack.push(tag.to_string());
            self.cache.push(text);
        } else {
            self.cleandoc.push(text);
            self.open_inline_tag(tag);
            if CDATA_ELEMENTS.contains(&tag) {
                self.cdata = None;
            }
        }
    }

    fn base_handle_startendtag(&mut self, tag: &str) {
        let t = self.starttag_text.clone().unwrap();
        self.handle_empty_tag(&t, self.ctx.is_block_level(tag));
    }

    fn base_handle_endtag(&mut self, tag: &str) {
        let text = self.endtag_text();
        if self.inraw {
            self.cache.push(text.clone());
            if self.stack.iter().any(|t| t == tag) {
                while let Some(top) = self.stack.pop() {
                    if top == tag {
                        break;
                    }
                }
            }
            if self.stack.is_empty() {
                let after_start = self.pos + text.len();
                if BLANK_LINE_RE.find(suffix_at(self.text, after_start)).ok().flatten().map(|m| m.start() == 0).unwrap_or(false) {
                    self.cache.push("\n".to_string());
                } else {
                    self.intail = true;
                }
                self.inraw = false;
                let raw = self.cache.join("");
                let ph = self.store(raw);
                self.cleandoc.push(ph);
                self.cleandoc.push("\n\n".to_string());
                self.cache.clear();
            }
        } else {
            self.cleandoc.push(text);
            self.close_inline_tag(tag);
        }
    }

    fn endtag_text(&self) -> String {
        // `line_offset + offset` is exactly the token start (`self.pos`).
        let start = self.pos;
        // `endendtag` (`>`) always matches: the caller verified a `>` ahead.
        let rel = self.text[start..].find('>').unwrap();
        self.text[start..start + rel + 1].to_string()
    }

    fn open_inline_tag(&mut self, tag: &str) {
        if !self.ctx.is_block_level(tag) && !VOID_TAGS.contains(&tag) {
            self.inline_stack.push(tag.to_string());
        }
    }

    fn close_inline_tag(&mut self, tag: &str) {
        if self.inline_stack.iter().any(|t| t == tag) {
            while let Some(top) = self.inline_stack.pop() {
                if top == tag {
                    break;
                }
            }
        }
    }

    fn inline_close_follows(&self, text: &str) -> bool {
        if self.inline_stack.is_empty() {
            return false;
        }
        let start = self.pos + text.len();
        let rest = suffix_at(self.text, start);
        let before: String = match INNER_BLANK_RE.find(rest).ok().flatten() {
            Some(m) => rest[..m.start()].to_string(),
            None => rest.to_string(),
        };
        self.inline_stack.iter().any(|tag| {
            let pat = format!("(?i)</\\s*{tag}\\s*>");
            crate::consts::compile(&pat).find(&before).ok().flatten().is_some()
        })
    }

    fn base_handle_data(&mut self, data: &str) {
        if self.intail && data.contains('\n') {
            self.intail = false;
        }
        if self.inraw {
            self.cache.push(data.to_string());
        } else {
            if INNER_BLANK_RE.find(data).ok().flatten().is_some() {
                self.inline_stack.clear();
            }
            self.cleandoc.push(data.to_string());
        }
    }

    fn base_handle_empty_tag(&mut self, data: &str, is_block: bool) {
        if self.inraw || self.intail {
            self.cache.push(data.to_string());
        } else if self.at_line_start(self.pos) && is_block {
            let mut owned = data.to_string();
            let after = self.pos + data.len();
            if BLANK_LINE_RE.find(suffix_at(self.text, after)).ok().flatten().map(|m| m.start() == 0).unwrap_or(false) {
                owned.push('\n');
            } else {
                self.intail = true;
            }
            let item = self.cleandoc.last().cloned().unwrap_or_default();
            if !item.ends_with("\n\n") && item.ends_with('\n') {
                self.cleandoc.push("\n".to_string());
            }
            let ph = self.store(owned);
            self.cleandoc.push(ph);
            self.cleandoc.push("\n\n".to_string());
        } else {
            self.cleandoc.push(data.to_string());
        }
    }

    fn handle_charref(&mut self, name: &str) {
        self.handle_empty_tag(&format!("&#{name};"), false);
    }

    fn handle_entityref(&mut self, name: &str) {
        self.handle_empty_tag(&format!("&{name};"), false);
    }

    fn handle_comment(&mut self, data: &str) {
        let text = format!("<!--{data}-->");
        let is_block = INNER_BLANK_RE.find(data).ok().flatten().is_some()
            || !self.inline_close_follows(&text);
        self.handle_empty_tag(&text, is_block);
    }

    fn handle_decl(&mut self, data: &str) {
        self.handle_empty_tag(&format!("<!{data}>"), true);
    }

    fn handle_pi(&mut self, data: &str) {
        self.handle_empty_tag(&format!("<?{data}?>"), true);
    }

    fn unknown_decl(&mut self, data: &str) {
        let end = if data.starts_with("CDATA[") { "]]>" } else { "]>" };
        self.handle_empty_tag(&format!("<![{data}{end}"), true);
    }
}

impl<'a, 'c> Extractor<'a, 'c> {
    /// Dispatching wrappers (base vs `md_in_html` override).
    fn handle_data(&mut self, data: &str) {
        if self.extra {
            self.extra_handle_data(data);
        } else {
            self.base_handle_data(data);
        }
    }

    fn handle_empty_tag(&mut self, data: &str, is_block: bool) {
        if self.extra {
            self.extra_handle_empty_tag(data, is_block);
        } else {
            self.base_handle_empty_tag(data, is_block);
        }
    }

    fn handle_endtag(&mut self, tag: &str) {
        if self.extra {
            self.extra_handle_endtag(tag);
        } else {
            self.base_handle_endtag(tag);
        }
    }
}
// ---- `HTMLExtractorExtra` (`md_in_html.py`) ----

/// Markdown `state` values.
const MD_BLOCK: u8 = 0;
const MD_SPAN: u8 = 1;
const MD_OFF: u8 = 2;

/// Block-level tags whose content only gets span-level parsing.
const SPAN_TAGS: &[&str] = &[
    "address", "dd", "dt", "h1", "h2", "h3", "h4", "h5", "h6", "legend", "li", "p", "summary",
    "td", "th",
];
/// Block-level tags which never get their content parsed.
const RAW_TAGS: &[&str] = &["canvas", "math", "option", "pre", "script", "style", "textarea"];

impl<'a, 'c> Extractor<'a, 'c> {
    fn block_tags(&self) -> Vec<String> {
        self.ctx
            .block_level_elements
            .iter()
            .filter(|t| {
                let t = t.as_str();
                !(SPAN_TAGS.contains(&t) || RAW_TAGS.contains(&t) || t == "hr")
            })
            .cloned()
            .collect()
    }

    fn span_and_block_tags(&self) -> Vec<String> {
        let mut v = self.block_tags();
        v.extend(SPAN_TAGS.iter().map(|s| s.to_string()));
        v
    }

    /// `close()`: handle unclosed markdown tags.
    fn close_extra(&mut self) {
        if self.extra && !self.mdstack.is_empty() {
            let tag = self.mdstack[0].clone();
            self.handle_endtag(&tag);
        }
    }

    /// `get_state(tag, attrs)`.
    fn extra_state(&self, tag: &str, attrs: &[(String, Option<String>)]) -> Option<u8> {
        let raw: Option<Option<String>> = attrs
            .iter()
            .find(|(k, _)| k == "markdown")
            .map(|(_, v)| v.clone());
        // Absent -> '0'; valueless `<tag markdown>` -> None (never equals
        // a state string, but counts as non-'0' for the parent rule).
        let mut md_attr: Option<String> = match raw {
            None => Some("0".to_string()),
            Some(v) => v,
        };
        if md_attr.as_deref() == Some("markdown") {
            md_attr = Some("1".to_string());
        }
        let parent_state: Option<u8> = self.mdstate.last().copied().flatten();
        if parent_state == Some(MD_OFF)
            || (parent_state == Some(MD_SPAN) && md_attr.as_deref() != Some("0"))
        {
            md_attr = parent_state.map(|s| {
                if s == MD_OFF {
                    "off".to_string()
                } else {
                    "span".to_string()
                }
            });
        }
        let block_tags = self.block_tags();
        let span_block = self.span_and_block_tags();
        if (md_attr.as_deref() == Some("1") && block_tags.iter().any(|t| t == tag))
            || (md_attr.as_deref() == Some("block") && span_block.iter().any(|t| t == tag))
        {
            return Some(MD_BLOCK);
        }
        if (md_attr.as_deref() == Some("1") && SPAN_TAGS.contains(&tag))
            || (md_attr.as_deref() == Some("span") && span_block.iter().any(|t| t == tag))
        {
            return Some(MD_SPAN);
        }
        if self.ctx.is_block_level(tag) {
            return Some(MD_OFF);
        }
        None
    }

    /// `treebuilder.data`.
    fn tb_data(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        if let Some(top) = self.tstack.last_mut() {
            if let Some(last) = top.children.last_mut() {
                match &mut last.tail {
                    Some(Text::Normal(t)) => t.push_str(s),
                    _ => last.tail = Some(Text::Normal(s.to_string())),
                }
            } else {
                match &mut top.text {
                    Some(Text::Normal(t)) => t.push_str(s),
                    _ => top.text = Some(Text::Normal(s.to_string())),
                }
            }
        }
    }

    /// Parse `name[=value]` pairs between tag-name end and tag end.
    fn parse_attrs_at(&self, from: usize, to: usize) -> Vec<(String, Option<String>)> {
        let mut out = Vec::new();
        let mut k = from;
        while k < to {
            let caps = match ATTRFIND.captures_from_pos(self.text, k) {
                Ok(Some(c)) if c.get(0).unwrap().start() == k => c,
                _ => break,
            };
            let name = caps.get(1).unwrap().as_str().to_string();
            // Group 3 is the whole value: `'...'` / `"..."` (strip the
            // quotes) or bare (as-is).
            let val = caps.get(3).map(|g| {
                let v = g.as_str();
                if v.len() >= 2
                    && ((v.starts_with('\'') && v.ends_with('\''))
                        || (v.starts_with('"') && v.ends_with('"')))
                {
                    v[1..v.len() - 1].to_string()
                } else {
                    v.to_string()
                }
            });
            // stdlib `parse_starttag` unescapes the value.
            let val = val.map(|v| Self::html_unescape_attr(&v));
            out.push((name, val));
            k = caps.get(0).unwrap().end();
        }
        out
    }
    /// stdlib `htmlparser.unescape` for attribute values: resolve
/// character/entity references (`&amp;`, `&#38;`, `&#x26;`).
fn html_unescape_attr(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        let end = tail.find(';');
        let rep = match end {
            Some(e) if e < 32 => {
                let body = &tail[1..e];
                if let Some(hex) = body.strip_prefix("#x").or_else(|| body.strip_prefix("#X")) {
                    u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
                } else if let Some(dec) = body.strip_prefix('#') {
                    dec.parse::<u32>().ok().and_then(char::from_u32)
                } else if let Some((_k, v)) = crate::entities_html5_gen::HTML5
                    .iter()
                    .find(|(k, _)| *k == body)
                {
                    // Table values are single chars.
                    v.chars().next()
                } else {
                    None
                }
                .map(|c| c.to_string())
            }
            _ => None,
        };
        match rep {
            Some(r) => {
                out.push_str(&r);
                rest = &tail[end.unwrap() + 1..];
            }
            None => {
                out.push('&');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// Serialize an element for stashing (attribute order preserved).
    /// Upstream uses `etree.tostring(method='html')` here, so serialize
    /// in HTML mode regardless of the render format.
    fn el_to_string(&self, el: &Element) -> String {
        crate::ser::serialize(el, crate::OutputFormat::Html)
    }
}

impl<'a, 'c> Extractor<'a, 'c> {
    /// `handle_starttag` override.
    fn extra_handle_starttag(&mut self, tag: &str, attrs: &[(String, Option<String>)]) {
        if tag == "hr" && (self.at_line_start(self.pos) || self.intail) {
            let has_md = attrs.iter().any(|(k, _)| k == "markdown");
            if has_md {
                let mut el = Element::new(tag);
                for (k, v) in attrs {
                    if k != "markdown" {
                        el.set_attr(k, &v.clone().unwrap_or_else(|| k.clone()));
                    }
                }
                let data = self.el_to_string(&el);
                self.handle_empty_tag(&data, true);
            } else {
                let t = self.starttag_text.clone().unwrap();
                self.handle_empty_tag(&t, true);
            }
            return;
        }
        if self.ctx.is_block_level(tag)
            && (self.at_line_start(self.pos)
                || self.intail
                || self.mdstarted.last().copied().unwrap_or(false))
        {
            // Valueless attributes become `key=key` BEFORE state lookup
            // (`<tag markdown>` counts as `markdown="1"`).
            let norm0: Vec<(String, String)> = attrs
                .iter()
                .map(|(k, v)| (k.clone(), v.clone().unwrap_or_else(|| k.clone())))
                .collect();
            let normrefs: Vec<(String, Option<String>)> =
                norm0.iter().map(|(k, v)| (k.clone(), Some(v.clone()))).collect();
            let state = self.extra_state(tag, &normrefs);
            if self.inraw || ((state.is_none() || state == Some(MD_OFF)) && self.mdstack.is_empty()) {
                // Fall back to default behavior (attrs ignored downstream).
                self.base_handle_starttag(tag);
            } else {
                if self.mdstack.iter().any(|t| t == "p") && self.ctx.is_block_level(tag) {
                    self.handle_endtag("p");
                }
                self.mdstate.push(state);
                self.mdstack.push(tag.to_string());
                self.mdstarted.push(true);
                let mut el = Element::new(tag);
                for (k, v) in attrs {
                    if k == "markdown" {
                        continue;
                    }
                    el.set_attr(k, &v.clone().unwrap_or_else(|| k.clone()));
                }
                if let Some(s) = state {
                    let sv = match s {
                        MD_BLOCK => "block",
                        MD_SPAN => "span",
                        _ => "off",
                    };
                    el.set_attr("markdown", sv);
                }
                self.tstack.push(el);
            }
        } else {
            // Span level tag.
            if self.inraw {
                self.base_handle_starttag(tag);
            } else {
                let text = self.starttag_text.clone().unwrap();
                if self.mdstate.last().copied().flatten() == Some(MD_OFF) {
                    let ph = self.store(text);
                    self.extra_handle_data(&ph);
                } else {
                    self.extra_handle_data(&text);
                }
                self.open_inline_tag(tag);
                if CDATA_ELEMENTS.contains(&tag) {
                    self.cdata = None;
                }
            }
        }
    }

    /// `handle_endtag` override.
    fn extra_handle_endtag(&mut self, tag: &str) {
        if self.ctx.is_block_level(tag) {
            if self.inraw {
                self.base_handle_endtag(tag);
            } else if self.mdstack.iter().any(|t| t == tag) {
                // Close element and any unclosed children.
                while let Some(item) = self.mdstack.pop() {
                    self.mdstate.pop();
                    self.mdstarted.pop();
                    let closed = self.tstack.pop().unwrap();
                    if let Some(top) = self.tstack.last_mut() {
                        top.children.push(closed);
                    } else {
                        self.tstack.push(closed);
                    }
                    if item == tag {
                        break;
                    }
                }
                if self.mdstack.is_empty() {
                    let element = self.tstack.pop().unwrap();
                    self.extra_close_element(element, tag);
                }
            } else {
                // Orphan closing tag: span level.
                let text = self.endtag_text();
                if self.mdstate.last().copied().flatten() == Some(MD_OFF) {
                    let ph = self.store(text);
                    self.extra_handle_data(&ph);
                } else {
                    self.extra_handle_data(&text);
                }
            }
        } else if self.inraw {
            self.base_handle_endtag(tag);
        } else {
            let text = self.endtag_text();
            if self.mdstate.last().copied().flatten() == Some(MD_OFF) {
                let ph = self.store(text);
                self.extra_handle_data(&ph);
            } else {
                self.extra_handle_data(&text);
            }
            self.close_inline_tag(tag);
        }
    }

    /// `handle_startendtag` override.
    fn extra_handle_startendtag(&mut self, tag: &str, attrs: &[(String, Option<String>)]) {
        // Only the `markdown`-carrying empty tag is rebuilt; anything
        // else passes through verbatim (upstream `if "markdown"`).
        if tag == "hr" && attrs.iter().any(|(k, _)| k == "markdown") {
            let mut el = Element::new(tag);
            for (k, v) in attrs {
                if k != "markdown" {
                    el.set_attr(k, &v.clone().unwrap_or_else(|| k.clone()));
                }
            }
            let data = self.el_to_string(&el);
            self.handle_empty_tag(&data, self.ctx.is_block_level(tag));
            return;
        }
        let t = self.starttag_text.clone().unwrap();
        self.handle_empty_tag(&t, self.ctx.is_block_level(tag));
    }

    /// `handle_data` override.
    fn extra_handle_data(&mut self, data: &str) {
        if self.intail && data.contains('\n') {
            self.intail = false;
        }
        if self.inraw || self.mdstack.is_empty() {
            self.base_handle_data(data);
        } else {
            if let Some(top) = self.mdstarted.last_mut() {
                *top = false;
            }
            self.tb_data(data);
        }
    }

    /// `handle_empty_tag` override.
    fn extra_handle_empty_tag(&mut self, data: &str, is_block: bool) {
        if self.inraw || self.mdstack.is_empty() {
            self.base_handle_empty_tag(data, is_block);
        } else if self.at_line_start(self.pos) && is_block {
            let ph = self.store(data.to_string());
            self.extra_handle_data(&format!("\n{ph}\n\n"));
        } else if self.mdstate.last().copied().flatten() == Some(MD_OFF) {
            let ph = self.store(data.to_string());
            self.extra_handle_data(&ph);
        } else {
            self.extra_handle_data(data);
        }
    }

    /// Flatten + stash a closed markdown element.
    fn extra_close_element(&mut self, element: Element, tag: &str) {
        let item = self.cleandoc.last().cloned().unwrap_or_default();
        if !item.ends_with("\n\n") && item.ends_with('\n') {
            self.cleandoc.push("\n".to_string());
        }
        // Work on stash-resident elements via keys so nested expansions
        // mutate the stored copy (mirrors shared Python references).
        let root_key = self.ctx.stash.len();
        self.ctx.stash.push(StashEntry::Tree(element));
        let mut current_key = root_key;
        let mut last: Vec<usize> = Vec::new();
        loop {
            // Take children snapshot, then drain them one by one.
            let kids: Vec<Element> = {
                let cur = self.tree_at(current_key);
                std::mem::take(&mut cur.children)
            };
            for mut child in kids {
                let text = {
                    let cur = self.tree_at(current_key);
                    cur.text.clone().map(|t| t.into_string()).unwrap_or_default()
                };
                let tail = child.tail.take().map(|t| t.into_string()).unwrap_or_default();
                let state = child.get_attr("markdown").unwrap_or("off").to_string();
                let mut tail = tail;
                if tail != "\n" {
                    tail = format!("\n{}", tail.trim_end_matches('\n'));
                }
                let mut text = text;
                if !text.ends_with("\n\n") {
                    text = format!("{}\n\n", text.trim_end_matches('\n'));
                }
                if state == "span" || state == "block" {
                    let key = self.ctx.stash.len();
                    self.ctx.stash.push(StashEntry::Tree(child));
                    {
                        let cur = self.tree_at(current_key);
                        cur.text = Some(Text::Normal(format!(
                            "{text}{}{tail}",
                            crate::consts::placeholder_html(key)
                        )));
                    }
                    last.push(key);
                } else {
                    strip_markdown_attrs(&mut child);
                    let key = self.ctx.stash.len();
                    self.ctx.stash.push(StashEntry::Tree(child));
                    {
                        let cur = self.tree_at(current_key);
                        cur.text = Some(Text::Normal(format!(
                            "{text}{}{tail}",
                            crate::consts::placeholder_html(key)
                        )));
                    }
                }
            }
            if last.is_empty() {
                break;
            }
            current_key = last.remove(0);
        }
        let ph = crate::consts::placeholder_html(root_key);
        self.cleandoc.push(ph);
        self.cleandoc.push("\n\n".to_string());
        let end_txt = format!("</{tag}>");
        let after = suffix_at(self.text, self.pos + end_txt.len());
        if !BLANK_LINE_RE.find(after).ok().flatten().map(|m| m.start() == 0).unwrap_or(false) {
            self.intail = true;
        }
    }

    /// `&mut` to a stash-resident tree element.
    fn tree_at(&mut self, key: usize) -> &mut Element {
        match &mut self.ctx.stash[key] {
            StashEntry::Tree(el) => el,
            _ => unreachable!("md_in_html stash holds elements here"),
        }
    }
}

/// Remove `markdown` attributes from an element and all descendants.
fn strip_markdown_attrs(el: &mut Element) {
    el.attrib.retain(|(k, _)| k != "markdown");
    for c in &mut el.children {
        strip_markdown_attrs(c);
    }
}

use std::sync::LazyLock;

macro_rules! hx {
    ($name:ident, $src:expr) => {
        static $name: LazyLock<F> = LazyLock::new(|| crate::consts::compile($src));
    };
}

hx!(STARTTAGOPEN, "<[a-zA-Z]|</>");
hx!(ENDTAGOPEN, "</[a-zA-Z]?");
hx!(PICLOSE, "\\?>");
hx!(CHARREF, "&#(?:[0-9]+|[xX][0-9a-fA-F]+)[^0-9a-fA-F]");
hx!(ENTITYREF, "&([a-zA-Z][-.a-zA-Z0-9]*);");
hx!(COMMENTCLOSE, "--!?>");
hx!(BLANK_LINE_RE, "^([ ]*\\n){2}");
hx!(INNER_BLANK_RE, "\\n[ \\t]*\\n");

fn dump_pat(name: &str) -> &'static str {
    match name {
        "locatestarttagend" => crate::patterns_gen::RE_HTML_STD_LOCATESTARTTAGEND_TOLERANT,
        "locatetagend" => crate::patterns_gen::RE_HTML_STD_LOCATETAGEND,
        "tagfind" => crate::patterns_gen::RE_HTML_STD_TAGFIND_TOLERANT,
        "attrfind" => crate::patterns_gen::RE_HTML_STD_ATTRFIND_TOLERANT,
        _ => unreachable!(),
    }
}

hx!(LOCATETAGEND, dump_pat("locatetagend"));
hx!(TAGFIND, dump_pat("tagfind"));
hx!(ATTRFIND, dump_pat("attrfind"));

/// `check_for_whole_start_tag`: `locatetagend` at `pos+1`, `-1` unless
/// the match ends with `>`.
fn check_whole_start_tag(text: &str, pos: usize) -> i64 {
    let m = match LOCATETAGEND.find_from_pos(text, pos + 1) {
        Ok(Some(m)) if m.start() == pos + 1 => m,
        _ => return -1,
    };
    let j = m.end();
    if !text[..j].ends_with('>') {
        return -1;
    }
    j as i64
}

fn starttagopen_at(text: &str, pos: usize) -> bool {
    STARTTAGOPEN
        .find_from_pos(text, pos)
        .ok()
        .flatten()
        .map(|m| m.start() == pos)
        .unwrap_or(false)
}

fn endtagopen_at(text: &str, pos: usize) -> bool {
    ENDTAGOPEN
        .find_from_pos(text, pos)
        .ok()
        .flatten()
        .map(|m| m.start() == pos)
        .unwrap_or(false)
}

/// Clamped suffix: Python slicing never panics on overrun.
fn suffix_at(text: &str, pos: usize) -> &str {
    if pos >= text.len() {
        ""
    } else {
        &text[pos..]
    }
}

/// Byte index just past the char at/after `pos` (char-boundary safe).
fn char_end(text: &str, pos: usize) -> usize {
    let mut e = pos.min(text.len());
    while !text.is_char_boundary(e) {
        e += 1;
    }
    e
}

/// Group 1 of `tagfind_tolerant` anchored at `pos`.
fn tag_name_at(text: &str, pos: usize) -> String {
    let caps = TAGFIND.captures_from_pos(text, pos).unwrap().unwrap();
    debug_assert_eq!(caps.get(0).unwrap().start(), pos);
    caps.get(1).unwrap().as_str().to_string()
}

/// `tagfind_end`: end of the whole `tagfind_tolerant` match at `pos`.
fn tagfind_end(text: &str, pos: usize) -> usize {
    let caps = TAGFIND.captures_from_pos(text, pos).unwrap().unwrap();
    debug_assert_eq!(caps.get(0).unwrap().start(), pos);
    caps.get(0).unwrap().end()
}
