//! Port of `markdown/extensions/def_list.py`
//! (`defindent` 85 via the shared indent runner with `dd`/`dl` awareness,
//! `deflist` 25 here).

use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};
use crate::patterns_gen as P;

fn re() -> fancy_regex::Regex {
    compile(P::RE_EXT_DEF_LIST_DEFLISTPROCESSOR_RE)
}

fn no_indent_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_DEF_LIST_DEFLISTPROCESSOR_NO_INDENT_RE)
}

/// `DefListProcessor.run`.
pub fn run(ctx: &mut Context, parent: &mut Element, blocks: &mut Vec<String>) -> bool {
    let raw = blocks.remove(0);
    let m = match re().find(&raw) {
        Ok(Some(m)) => m,
        _ => {
            blocks.insert(0, raw);
            return false;
        }
    };
    let (ms, me) = (m.start(), m.end());
    let caps = re().captures(&raw).unwrap().unwrap();
    let terms: Vec<String> = raw[..ms]
        .split('\n')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();
    let block = raw[me..].to_string();
    let no_indent = no_indent_re()
        .find(&block)
        .ok()
        .flatten()
        .map(|m| m.start() == 0)
        .unwrap_or(false);
    let (d, the_rest) = if no_indent {
        (block, None)
    } else {
        let (a, b) = crate::block::detab(&block, ctx.tab_length);
        (a, if b.is_empty() { None } else { Some(b) })
    };
    let d = if d.is_empty() {
        caps.get(2).unwrap().as_str().to_string()
    } else {
        format!("{}\n{d}", caps.get(2).unwrap().as_str())
    };
    let sib_tag = parent.children.last().map(|c| c.tag.clone());
    if terms.is_empty() && sib_tag.is_none() {
        blocks.insert(0, raw);
        return false;
    }
    let mut terms = terms;
    let state: &str;
    if terms.is_empty() && sib_tag.as_deref() == Some("p") {
        state = "looselist";
        let sib = parent.children.pop().unwrap();
        terms = sib
            .text
            .map(|t| {
                t.into_string()
                    .split('\n')
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
    } else {
        state = "list";
    }
    let dl_idx = match parent.children.last() {
        Some(s) if s.tag == "dl" => parent.children.len() - 1,
        _ => {
            parent.children.push(Element::new("dl"));
            parent.children.len() - 1
        }
    };
    // Existing list whose last item already has content: loose.
    let mut state = state;
    {
        let dl = &parent.children[dl_idx];
        if terms.is_empty()
            && !dl.children.is_empty()
            && dl.children[dl.children.len() - 1].tag == "dd"
            && !dl.children[dl.children.len() - 1].children.is_empty()
        {
            state = "looselist";
        }
    }
    for term in &terms {
        let mut dt = Element::new("dt");
        dt.text = Some(Text::Normal(term.clone()));
        parent.children[dl_idx].children.push(dt);
    }
    ctx.bst_set(state);
    {
        let dl = &mut parent.children[dl_idx];
        dl.children.push(Element::new("dd"));
        let dd_idx = dl.children.len() - 1;
        crate::block::parse_blocks(ctx, &mut dl.children[dd_idx], vec![d]);
    }
    ctx.bst_reset();
    if let Some(rest) = the_rest {
        if !rest.is_empty() {
            blocks.insert(0, rest);
        }
    }
    true
}
