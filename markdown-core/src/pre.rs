//! `NormalizeWhitespace` + `HtmlBlockPreprocessor` (`preprocessors.py`).

use crate::context::{Context, StashEntry};
use crate::html::extract_html;

/// `NormalizeWhitespace.run`: join, strip STX/ETX, normalize newlines,
/// pad, expand tabs, collapse whitespace-only lines.
pub fn normalize_whitespace(lines: &[String], tab_length: usize) -> Vec<String> {
    let mut source = lines.join("\n");
    source = source.replace(['\u{2}', '\u{3}'], "");
    source = source.replace("\r\n", "\n").replace('\r', "\n") + "\n\n";
    source = expand_tabs(&source, tab_length);
    // `re.sub(r'(?<=\n) +\n', '\n', source)`: blank out whitespace-only lines.
    let mut out = String::with_capacity(source.len());
    let mut line_start = true;
    let mut spaces = 0;
    for ch in source.chars() {
        if line_start {
            if ch == ' ' {
                spaces += 1;
                continue;
            }
            if ch == '\n' {
                // Whitespace-only line: drop the buffered spaces.
                spaces = 0;
                out.push('\n');
                continue;
            }
            for _ in 0..spaces {
                out.push(' ');
            }
            spaces = 0;
            line_start = false;
            out.push(ch);
        } else if ch == '\n' {
            line_start = true;
            out.push(ch);
        } else {
            out.push(ch);
        }
    }
    // A trailing partial line's buffered spaces (no newline follows) are kept.
    if line_start {
        for _ in 0..spaces {
            out.push(' ');
        }
    }
    out.split('\n').map(str::to_string).collect()
}

/// Python `str.expandtabs`: per-char columns, reset on `\n`/`\r`.
pub fn expand_tabs(s: &str, tab_length: usize) -> String {
    if !s.contains('\t') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len() + 8);
    let mut col: usize = 0;
    for ch in s.chars() {
        match ch {
            '\t' => {
                let n = tab_length - (col % tab_length);
                for _ in 0..n {
                    out.push(' ');
                }
                col += n;
            }
            '\n' | '\r' => {
                col = 0;
                out.push(ch);
            }
            _ => {
                col += 1;
                out.push(ch);
            }
        }
    }
    out
}

/// `HtmlBlockPreprocessor.run`: stash HTML blocks, return clean lines.
pub fn html_block(ctx: &mut Context, lines: Vec<String>) -> Vec<String> {
    let source = lines.join("\n");
    let cleandoc = extract_html(ctx, &source);
    cleandoc.join("").split('\n').map(str::to_string).collect()
}

/// Store a raw-HTML string in the stash (mirrors `HtmlStash.store`).
pub fn stash_html(ctx: &mut Context, html: String) -> String {
    ctx.stash_store(StashEntry::Html(html))
}
