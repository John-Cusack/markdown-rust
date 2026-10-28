//! Port of `markdown/extensions/md_in_html.py`
//! (`MarkdownInHtmlProcessor`, priority 105).

use crate::context::{Context, StashEntry};
use crate::model::{Element, Text};

/// Placeholder prefix/suffix for HTML stash entries.
fn ph_start(data: &str) -> Option<usize> {
    let p = "\u{2}wzxhzdk:";
    let i = data.find(p)?;
    let k = i + p.len();
    let mut e = k;
    while e < data.len() && data.as_bytes()[e].is_ascii_digit() {
        e += 1;
    }
    if e < data.len() && data.as_bytes()[e] == 3 && e > k {
        Some(i)
    } else {
        None
    }
}

fn ph_key(data: &str, at: usize) -> Option<(usize, usize)> {
    let p = "\u{2}wzxhzdk:";
    let k = at + p.len();
    let mut e = k;
    while e < data.len() && data.as_bytes()[e].is_ascii_digit() {
        e += 1;
    }
    if e < data.len() && data.as_bytes()[e] == 3 && e > k {
        Some((data[k..e].parse().ok()?, e + 1))
    } else {
        None
    }
}

/// `MarkdownInHtmlProcessor.run`.
pub fn run_markdown_block(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    let first = match blocks.first() {
        Some(b) => b.clone(),
        None => return false,
    };
    let Some(at) = ph_start(&first) else {
        return false;
    };
    if at != 0 {
        return false;
    }
    let Some((key, end)) = ph_key(&first, at) else {
        return false;
    };
    if key >= ctx.stash.len() {
        return false;
    }
    let is_tree = matches!(ctx.stash[key], StashEntry::Tree(_));
    if !is_tree {
        return false;
    }
    // Take the element out of the stash (processed in place below).
    let element = match std::mem::replace(&mut ctx.stash[key], StashEntry::Html(String::new())) {
        StashEntry::Tree(el) => el,
        _ => unreachable!(),
    };
    blocks.remove(0);
    parent.children.push(element);
    let idx = parent.children.len() - 1;
    parse_element_content(ctx, &mut parent.children[idx]);
    let rest = first[end..].to_string();
    if !rest.is_empty() {
        blocks.insert(0, rest);
    }
    true
}

/// Recursively parse an element's content as Markdown.
pub fn parse_element_content(ctx: &mut Context, element: &mut Element) {
    // Pop `markdown`, default `off`.
    let mut md_attr = "off".to_string();
    let mut kept = Vec::new();
    for (k, v) in std::mem::take(&mut element.attrib) {
        if k == "markdown" {
            md_attr = v;
        } else {
            kept.push((k, v));
        }
    }
    element.attrib = kept;
    if md_attr == "block" {
        if let Some(t) = element.text.take() {
            let s = t.into_string();
            let block = s.trim_end_matches('\n').to_string();
            if !block.is_empty() {
                let parts: Vec<String> = block.split("\n\n").map(str::to_string).collect();
                crate::block::parse_blocks(ctx, element, parts);
            }
        }
    } else if md_attr == "span" {
        let block = element.text.take().map(|t| t.into_string()).unwrap_or_default();
        let mut child: Option<usize> = None;
        let mut start = 0usize;
        // Scan for HTML placeholders.
        let mut poses: Vec<(usize, usize, usize)> = Vec::new();
        {
            let mut i = 0usize;
            while i < block.len() {
                match block[i..].find("\u{2}wzxhzdk:") {
                    Some(r) => {
                        let at = i + r;
                        if let Some((key, end)) = ph_key(&block, at) {
                            poses.push((at, key, end));
                            i = end;
                        } else {
                            i = at + 1;
                        }
                    }
                    None => break,
                }
            }
        }
        for (at, key, _end) in poses {
            let is_el = key < ctx.stash.len() && matches!(ctx.stash[key], StashEntry::Tree(_));
            // Upstream names the *match start* `end`: gaps run from the
            // previous match start, and the trailing rest re-includes
            // the last placeholder (resolved post-serialize, shielding
            // newlines from prettify).
            let gap = block[start..at].to_string();
            if is_el {
                let el = match std::mem::replace(&mut ctx.stash[key], StashEntry::Html(String::new())) {
                    StashEntry::Tree(el) => el,
                    _ => unreachable!(),
                };
                match child {
                    None => {
                        let cur = element.text.take().map(|t| t.into_string()).unwrap_or_default();
                        element.text = Some(Text::Normal(format!("{cur}{gap}")));
                    }
                    Some(ci) => {
                        let cur = element.children[ci].tail.take().map(|t| t.into_string()).unwrap_or_default();
                        element.children[ci].tail = Some(Text::Normal(format!("{cur}{gap}")));
                    }
                }
                element.children.push(el);
                let ci = element.children.len() - 1;
                parse_element_content(ctx, &mut element.children[ci]);
                if element.children[ci].tail.is_none() {
                    element.children[ci].tail = Some(Text::Normal(String::new()));
                }
                child = Some(ci);
            } else {
                match child {
                    None => {
                        let cur = element.text.take().map(|t| t.into_string()).unwrap_or_default();
                        element.text = Some(Text::Normal(format!("{cur}{gap}")));
                    }
                    Some(ci) => {
                        let cur = element.children[ci].tail.take().map(|t| t.into_string()).unwrap_or_default();
                        element.children[ci].tail = Some(Text::Normal(format!("{cur}{gap}")));
                    }
                }
            }
            start = at;
        }
        // `start` is the last match's start: the rest re-includes its
        // placeholder text (upstream `block[start:]`).
        let rest = block[start..].to_string();
        match child {
            None => {
                let cur = element.text.take().map(|t| t.into_string()).unwrap_or_default();
                if !(cur.is_empty() && rest.is_empty()) {
                    element.text = Some(Text::Normal(format!("{cur}{rest}")));
                }
            }
            Some(ci) => {
                let cur = element.children[ci].tail.take().map(|t| t.into_string()).unwrap_or_default();
                element.children[ci].tail = Some(Text::Normal(format!("{cur}{rest}")));
            }
        }
    } else {
        // Disable inline parsing: wrap text/tails in `AtomicString`.
        match element.text.take() {
            None => element.text = Some(Text::Atomic(String::new())),
            Some(Text::Atomic(s)) => element.text = Some(Text::Atomic(s)),
            Some(Text::Normal(s)) => element.text = Some(Text::Atomic(s)),
        }
        for i in 0..element.children.len() {
            parse_element_content(ctx, &mut element.children[i]);
            if let Some(t) = element.children[i].tail.take() {
                let s = t.into_string();
                element.children[i].tail = Some(Text::Atomic(s));
            }
        }
    }
}
