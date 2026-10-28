//! `PrettifyTreeprocessor` + `UnescapeTreeprocessor` (`treeprocessors.py`).

use crate::consts::RE_UNESCAPE;
use crate::context::Context;
use crate::model::Text;


/// `PrettifyTreeprocessor._prettifyETree` (recursive).
fn prettify_el(ctx: &Context, el: &mut crate::model::Element) {
    if ctx.is_block_level(&el.tag) && el.tag != "code" && el.tag != "pre" {
        let first_is_block = el
            .children
            .first()
            .map(|c| ctx.is_block_level(&c.tag))
            .unwrap_or(false);
        let text_blank = el
            .text
            .as_ref()
            .map(|t| t.as_str().trim().is_empty())
            .unwrap_or(true);
        if text_blank && first_is_block {
            el.text = Some(Text::Normal("\n".to_string()));
        }
        for child in el.children.iter_mut() {
            if ctx.is_block_level(&child.tag) {
                prettify_el(ctx, child);
            }
        }
    }
    let tail_blank = el
        .tail
        .as_ref()
        .map(|t| t.as_str().trim().is_empty())
        .unwrap_or(true);
    if tail_blank {
        el.tail = Some(Text::Normal("\n".to_string()));
    }
}

/// `PrettifyTreeprocessor.run`.
pub fn prettify(ctx: &Context, root: &mut crate::model::Element) {
    prettify_el(ctx, root);
    // `br` tails.
    let mut stack = vec![root as *mut crate::model::Element];
    while let Some(ptr) = stack.pop() {
        let el = unsafe { &mut *ptr };
        for child in el.children.iter_mut() {
            if child.tag == "br" {
                let blank = child
                    .tail
                    .as_ref()
                    .map(|t| t.as_str().trim().is_empty())
                    .unwrap_or(true);
                if blank {
                    child.tail = Some(Text::Normal("\n".to_string()));
                } else {
                    let rest = child.tail.take().unwrap().into_string();
                    child.tail = Some(Text::Normal(format!("\n{rest}")));
                }
            }
            stack.push(child as *mut crate::model::Element);
        }
    }
    // `pre > code` cleanup.
    let mut stack = vec![root as *mut crate::model::Element];
    while let Some(ptr) = stack.pop() {
        let el = unsafe { &mut *ptr };
        if el.tag == "pre" && el.children.len() == 1 && el.children[0].tag == "code" {
            let code = &mut el.children[0];
            if code.children.is_empty() {
                if let Some(t) = code.text.take() {
                    let s = t.into_string();
                    code.text = Some(Text::Atomic(format!("{}\n", s.trim_end())));
                }
            }
        }
        for child in el.children.iter_mut() {
            stack.push(child as *mut crate::model::Element);
        }
    }
}

/// `UnescapeTreeprocessor`: `STX digits ETX` -> chr, on text/tail/attrs
/// of every element except `code` text.
pub fn unescape(_ctx: &Context, root: &mut crate::model::Element) {
    let mut stack = vec![root as *mut crate::model::Element];
    while let Some(ptr) = stack.pop() {
        let el = unsafe { &mut *ptr };
        if el.tag != "code" {
            if let Some(t) = el.text.take() {
                el.text = Some(unescape_text(t));
            }
        }
        if let Some(t) = el.tail.take() {
            el.tail = Some(unescape_text(t));
        }
        for (k, v) in el.attrib.iter_mut() {
            let _ = k;
            *v = unescape_str(v);
        }
        for child in el.children.iter_mut() {
            stack.push(child as *mut crate::model::Element);
        }
    }
}

fn unescape_str(s: &str) -> String {
    RE_UNESCAPE
        .replace_all(s, |caps: &fancy_regex::Captures| {
            let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
            char::from_u32(n).unwrap_or('\u{FFFD}').to_string()
        })
        .into_owned()
}

/// Upstream-exact atomicity: `RE.sub` returns the input object unchanged
/// when nothing matches (atomic bit preserved) and a plain `str` when a
/// substitution fires (bit lost). Post-unescape the bit is unobservable
/// downstream (no more inline passes), but extensions probing `AtomicString`
/// (abbr) run *before* unescape, so both orders agree in practice; the
/// equiv corpus pins the rest.
fn unescape_text(t: Text) -> Text {
    let hit = RE_UNESCAPE.find(t.as_str()).ok().flatten().is_some();
    if !hit {
        return t;
    }
    let s = t.into_string();
    Text::Normal(
        RE_UNESCAPE
            .replace_all(&s, |caps: &fancy_regex::Captures| {
                let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
                char::from_u32(n).unwrap_or('\u{FFFD}').to_string()
            })
            .into_owned(),
    )
}
