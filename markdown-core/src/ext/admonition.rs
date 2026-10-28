//! Port of `markdown/extensions/admonition.py`
//! (`AdmonitionProcessor`, priority 105).
//!
//! Upstream threads `current_sibling`/`content_indent` from `test()` to
//! `run()`; this port folds the sibling search and consume into the single
//! `run` call (net effect per block is identical: search, detab, consume).

use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};
use crate::patterns_gen as P;

fn re() -> fancy_regex::Regex {
    compile(P::RE_EXT_ADMONITION_ADMONITIONPROCESSOR_RE)
}

fn re_spaces() -> fancy_regex::Regex {
    compile(P::RE_EXT_ADMONITION_ADMONITIONPROCESSOR_RE_SPACES)
}

fn nav<'x>(root: &'x Element, path: &[usize]) -> &'x Element {
    let mut n = root;
    for &i in path {
        n = &n.children[i];
    }
    n
}

fn nav_mut<'x>(root: &'x mut Element, path: &[usize]) -> &'x mut Element {
    let mut n = root;
    for &i in path {
        n = &mut n.children[i];
    }
    n
}

/// `AdmonitionProcessor.run` (returns hit).
pub fn run(ctx: &mut Context, parent: &mut Element, blocks: &mut Vec<String>) -> bool {
    let block = blocks.remove(0);
    if let Some(m) = re().find(&block).ok().flatten() {
        let (ms, me) = (m.start(), m.end());
        if ms > 0 {
            crate::block::parse_blocks(ctx, parent, vec![block[..ms].to_string()]);
            if ctx.declined {
                return true;
            }
        }
        let rest = block[me..].to_string();
        let (content, the_rest) = crate::block::detab(&rest, ctx.tab_length);
        let caps = re().captures(&block).unwrap().unwrap();
        let (klass, title) = get_class_and_title(&caps);
        let mut div = Element::new("div");
        div.set_attr("class", &format!("admonition {klass}"));
        if let Some(title) = title {
            let mut p = Element::new("p");
            p.text = Some(Text::Normal(title));
            p.set_attr("class", "admonition-title");
            div.children.push(p);
        }
        let div_idx = parent.children.len();
        parent.children.push(div);
        crate::block::parse_chunk(ctx, &mut parent.children[div_idx], &content);
        if ctx.declined {
            return true;
        }
        if !the_rest.is_empty() {
            blocks.insert(0, the_rest);
        }
        return true;
    }
    // Continuation of a previous admonition (`parse_content` search).
    let tab = ctx.tab_length;
    let Some((sib_path, content, the_rest)) = find_sibling(parent, &block, tab) else {
        blocks.insert(0, block);
        return false;
    };
    // Sibling is a list item, but its content needs wrapping in `<p>`.
    {
        let needs_p = {
            let sib = nav(parent, &sib_path);
            (sib.tag == "li" || sib.tag == "dd")
                && sib.text.as_ref().map(|t| !t.as_str().is_empty()).unwrap_or(false)
        };
        if needs_p {
            let sib = nav_mut(parent, &sib_path);
            let text = sib.text.take().unwrap().into_string();
            let mut p = Element::new("p");
            p.text = Some(Text::Normal(text));
            sib.children.insert(0, p);
        }
    }
    {
        let sib = nav_mut(parent, &sib_path);
        crate::block::parse_chunk(ctx, sib, &content);
    }
    if ctx.declined {
        return true;
    }
    if !the_rest.is_empty() {
        blocks.insert(0, the_rest);
    }
    true
}

/// Mirror of `parse_content` (no cross-call state: search and consume inline).
/// Returns `(sibling_path, content, the_rest)`.
fn find_sibling(parent: &Element, block: &str, tab: usize) -> Option<(Vec<usize>, String, String)> {
    let sib_idx = parent.children.len().checked_sub(1)?;
    {
        let sib = &parent.children[sib_idx];
        if sib.tag != "div"
            || sib
                .get_attr("class")
                .map(|s| !s.contains("admonition"))
                .unwrap_or(true)
        {
            return None;
        }
    }
    // Descend into trailing lists while the block is doubly indented.
    // Each round steps list -> item (upstream: `sibling =
    // lastChild(last_child)`); the item's own last child is only
    // inspected on the next round.
    let last_child_of = |parent: &Element, path: &[usize]| -> Option<(String, Vec<usize>)> {
        let n = nav(parent, path);
        n.children.len().checked_sub(1).map(|i| {
            let mut p = path.to_vec();
            p.push(i);
            (n.children[i].tag.clone(), p)
        })
    };
    let mut cur_path: Vec<usize> = vec![sib_idx];
    let mut cur_block = block.to_string();
    let mut indent = 0usize;
    while let Some((lc_tag, lc_path)) = last_child_of(parent, &cur_path) {
        if !cur_block.starts_with(&" ".repeat(tab * 2))
            || (lc_tag != "ul" && lc_tag != "ol" && lc_tag != "dl")
        {
            break;
        }
        // Step to the item (last child of the list); an empty list
        // ends the search with no sibling (mirrors upstream `None`).
        let (_, item_path) = last_child_of(parent, &lc_path)?;
        cur_path = item_path;
        cur_block = cur_block[tab..].to_string();
        indent += tab;
    }
    if !cur_block.starts_with(&" ".repeat(tab)) {
        return None;
    }
    indent += tab;
    let (content, the_rest) = crate::block::detab(block, indent);
    Some((cur_path, content, the_rest))
}

fn get_class_and_title(caps: &fancy_regex::Captures) -> (String, Option<String>) {
    let mut klass = caps.get(1).unwrap().as_str().to_lowercase();
    klass = re_spaces().replace_all(&klass, " ").into_owned();
    let title = caps.get(2).map(|g| g.as_str().to_string());
    match title {
        None => {
            let mut t = klass.split(' ').next().unwrap_or("").to_string();
            if let Some(first) = t.get_mut(..1) {
                first.make_ascii_uppercase();
            }
            (klass, Some(t))
        }
        Some(t) if t.is_empty() => (klass, None),
        Some(t) => (klass, Some(t)),
    }
}
