//! `RawHtmlPostprocessor` + `AndSubstitutePostprocessor` (`postprocessors.py`).

use crate::consts::AMP_SUBSTITUTE;
use crate::context::{Context, StashEntry};

/// `RawHtmlPostprocessor.run`: restore stashed raw HTML.
pub fn raw_html(ctx: &Context, text: &str) -> String {
    if ctx.stash.is_empty() {
        return text.to_string();
    }
    // `<p>{HTML_PH}</p>|{HTML_PH}` — leftmost scan with `substitute_match`.
    let mut out = String::new();
    let mut rest = text;
    loop {
        // Find the next placeholder (either wrapped or bare).
        let (idx, key, wrapped) = match find_placeholder(rest) {
            Some(t) => t,
            None => {
                out.push_str(rest);
                break;
            }
        };
        out.push_str(&rest[..idx]);
        let mlen = if wrapped {
            "<p>".len() + ph_len(key) + "</p>".len()
        } else {
            ph_len(key)
        };
        let matched = rest[idx..idx + mlen].to_string();
        rest = &rest[idx + mlen..];
        if key >= ctx.stash.len() {
            // Guard (`key >= html_counter`): restore the whole match
            // verbatim, exactly like upstream's `return m.group(0)`.
            out.push_str(&matched);
            continue;
        }
        let html = stash_to_string(ctx, key);
        if !wrapped || is_block_level_html(ctx, &html) {
            // Recurse: restored HTML may itself contain placeholders.
            let mut owned = html;
            while let Some((i2, k2, w2)) = find_placeholder(&owned) {
                let ml = if w2 {
                    "<p>".len() + ph_len(k2) + "</p>".len()
                } else {
                    ph_len(k2)
                };
                if k2 >= ctx.stash.len() {
                    break;
                }
                let inner = stash_to_string(ctx, k2);
                let replacement = if !w2 || is_block_level_html(ctx, &inner) {
                    inner
                } else {
                    format!("<p>{inner}</p>")
                };
                owned = format!("{}{}{}", &owned[..i2], replacement, &owned[i2 + ml..]);
            }
            out.push_str(&owned);
        } else {
            out.push_str(&format!("<p>{html}</p>"));
        }
    }
    out
}

/// Length of `HTML_PLACEHOLDER % key` in bytes: STX + "wzxhzdk:" (8) +
/// digits + ETX. All ASCII.
fn ph_len(key: usize) -> usize {
    1 + 8 + key.to_string().len() + 1
}

/// Find the next `<p>PH</p>` or bare `PH` in `s`. Returns
/// (byte_index, key, wrapped).
fn find_placeholder(s: &str) -> Option<(usize, usize, bool)> {
    let mut best: Option<(usize, usize, bool)> = None;
    // Bare placeholder scan.
    let mut i = 0;
    while i < s.len() {
        let rel = s[i..].find('\u{2}');
        let j = match rel {
            Some(r) => i + r,
            None => break,
        };
        if s[j..].starts_with("\u{2}wzxhzdk:") {
            let k = j + "\u{2}wzxhzdk:".len();
            let mut e = k;
            while e < s.len() && s.as_bytes()[e].is_ascii_digit() {
                e += 1;
            }
            if e < s.len() && s.as_bytes()[e] == 3 && e > k {
                let key: usize = s[k..e].parse().ok()?;
                let wrapped = j >= 3
                    && s[..j].ends_with("<p>")
                    && s[j + (e - j + 1)..].starts_with("</p>");
                let idx = if wrapped { j - 3 } else { j };
                match best {
                    Some((bi, _, _)) if bi <= idx => {}
                    _ => best = Some((idx, key, wrapped)),
                };
                i = e + 1;
                continue;
            }
        }
        i = j + 1;
    }
    best
}

fn stash_to_string(ctx: &Context, key: usize) -> String {
    match &ctx.stash[key] {
        StashEntry::Html(h) => h.clone(),
        // `md_in_html` stores Elements: serialize via the core serializer.
        StashEntry::Tree(el) => crate::ser::serialize(el, ctx.output_format),
    }
}

fn is_block_level_html(ctx: &Context, html: &str) -> bool {
    // `BLOCK_LEVEL_REGEX = ^\<\/?([^ >]+)` (normalized: `^</?([^ >]+)`).
    let rest = html.strip_prefix('<').unwrap_or(html);
    let rest = rest.strip_prefix('/').unwrap_or(rest);
    let name: String = rest.chars().take_while(|&c| c != ' ' && c != '>').collect();
    let name = name.trim_end_matches('>');
    let name = name.split([' ', '\t', '\n', '\r', '/']).next().unwrap_or("");
    if name.is_empty() {
        return false;
    }
    if name.starts_with(['!', '?', '@', '%']) {
        return true;
    }
    ctx.is_block_level(name)
}

/// `AndSubstitutePostprocessor.run`.
pub fn amp_substitute(text: &str) -> String {
    text.replace(AMP_SUBSTITUTE, "&")
}
