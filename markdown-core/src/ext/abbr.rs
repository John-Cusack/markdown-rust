//! Port of `markdown/extensions/abbr.py`
//! (`abbr` block processor 16 harvests definitions; `abbr`
//! treeprocessor 7 substitutes them; wired in `pipeline`).
//!
//! Glossary seeding mirrors `extendMarkdown` + `reset`: per-render
//! `abbrs` start from the `glossary` config.

use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};
use crate::patterns_gen as P;

fn block_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_ABBR_ABBRBLOCKPROCESSOR_RE)
}

/// Seed per-render `abbrs` from the `glossary` config.
pub fn seed(ctx: &mut Context) {
    ctx.abbrs.clear();
    let seeded: Vec<(String, String)> = match ctx.ext_config("abbr", "glossary") {
        Some(crate::ConfigValue::StrMap(pairs)) => pairs.clone(),
        _ => Vec::new(),
    };
    for (k, v) in seeded {
        ctx.abbrs.insert(k, v);
    }
}

/// `AbbrBlockprocessor.run` (test always true; restores on no match).
pub fn run_block(ctx: &mut Context, _parent: &mut Element, blocks: &mut Vec<String>) -> bool {
    let block = blocks.remove(0);
    let re = block_re();
    let m = match re.find(&block) {
        Ok(Some(m)) => m,
        _ => {
            blocks.insert(0, block);
            return false;
        }
    };
    let caps = re.captures(&block).unwrap().unwrap();
    let abbr = caps.get(1).unwrap().as_str().trim().to_string();
    let title = caps.get(2).unwrap().as_str().trim().to_string();
    if !title.is_empty() && !abbr.is_empty() {
        if title == "''" || title == "\"\"" {
            ctx.abbrs.remove(&abbr);
        } else {
            ctx.abbrs.insert(abbr, title);
        }
        if !block[m.end()..].trim().is_empty() {
            blocks.insert(0, block[m.end()..].trim_start_matches('\n').to_string());
        }
        if !block[..m.start()].trim().is_empty() {
            blocks.insert(0, block[..m.start()].trim_end_matches('\n').to_string());
        }
        return true;
    }
    blocks.insert(0, block);
    false
}

/// `AbbrTreeprocessor.run`.
pub fn run_tree(ctx: &Context, root: &mut Element) {
    if ctx.abbrs.is_empty() {
        return;
    }
    let mut keys: Vec<&String> = ctx.abbrs.keys().collect();
    keys.sort_by_key(|a| std::cmp::Reverse(a.chars().count()));
    let pat = format!(
        "\\b(?:{})\\b",
        keys.iter()
            .map(|k| regex::escape(k))
            .collect::<Vec<_>>()
            .join("|")
    );
    let re = crate::consts::compile(&pat);
    iter_element(ctx, root, &re, &[]);
}

/// Post-order walk with parent paths (children reversed first, like
/// upstream `for child in reversed(el)`).
fn iter_element(ctx: &Context, root: &mut Element, re: &fancy_regex::Regex, path: &[usize]) {
    // Children first (reversed).
    let n = {
        let mut node: &mut Element = &mut *root;
        for &i in path {
            node = &mut node.children[i];
        }
        node.children.len()
    };
    for ri in 0..n {
        let i = n - 1 - ri;
        let mut sub = path.to_vec();
        sub.push(i);
        iter_element(ctx, root, re, &sub);
    }
    // Text of this node.
    let text = {
        let mut node: &mut Element = &mut *root;
        for &i in path {
            node = &mut node.children[i];
        }
        node.text.clone()
    };
    if let Some(t) = text {
        if !t.is_atomic() {
            let mut text_s = t.into_string();
            let matches: Vec<(usize, usize, String)> = re
                .captures_iter(&text_s)
                .filter_map(|r| r.ok())
                .map(|c| {
                    let m = c.get(0).unwrap();
                    (m.start(), m.end(), m.as_str().to_string())
                })
                .collect();
            for (ms, me) in matches.iter().rev().map(|(s, e, _)| (*s, *e)) {
                let key = text_s[ms..me].to_string();
                let title = match ctx.abbrs.get(&key) {
                    Some(t) if !t.is_empty() => t.clone(),
                    _ => continue,
                };
                let mut abbr = Element::new("abbr");
                abbr.set_attr("title", &title);
                abbr.text = Some(Text::Atomic(key));
                abbr.tail = Some(Text::Normal(text_s[me..].to_string()));
                {
                    let mut node: &mut Element = &mut *root;
                    for &i in path {
                        node = &mut node.children[i];
                    }
                    node.children.insert(0, abbr);
                }
                text_s = text_s[..ms].to_string();
            }
            let mut node: &mut Element = &mut *root;
            for &i in path {
                node = &mut node.children[i];
            }
            node.text = Some(Text::Normal(text_s));
        }
    }
    // Tail (needs the parent).
    if !path.is_empty() {
        let (parent_path, idx) = (&path[..path.len() - 1], path[path.len() - 1]);
        let tail = {
            let mut node: &mut Element = &mut *root;
            for &i in parent_path {
                node = &mut node.children[i];
            }
            node.children[idx].tail.clone()
        };
        if let Some(t) = tail {
            if !t.is_atomic() {
                let mut tail_s = t.into_string();
                let matches: Vec<(usize, usize)> = re
                    .captures_iter(&tail_s)
                    .filter_map(|r| r.ok())
                    .map(|c| {
                        let m = c.get(0).unwrap();
                        (m.start(), m.end())
                    })
                    .collect();
                let mut insert_at = idx + 1;
                for (ms, me) in matches.iter().rev().map(|(s, e)| (*s, *e)) {
                    let key = tail_s[ms..me].to_string();
                    let title = match ctx.abbrs.get(&key) {
                        Some(t) if !t.is_empty() => t.clone(),
                        _ => continue,
                    };
                    let mut abbr = Element::new("abbr");
                    abbr.set_attr("title", &title);
                    abbr.text = Some(Text::Atomic(key));
                    abbr.tail = Some(Text::Normal(tail_s[me..].to_string()));
                    let mut node: &mut Element = &mut *root;
                    for &i in parent_path {
                        node = &mut node.children[i];
                    }
                    node.children.insert(insert_at, abbr);
                    tail_s = tail_s[..ms].to_string();
                    let _ = insert_at;
                    insert_at = idx + 1;
                }
                let mut node: &mut Element = &mut *root;
                for &i in parent_path {
                    node = &mut node.children[i];
                }
                node.children[idx].tail = Some(Text::Normal(tail_s));
            }
        }
    }
}
