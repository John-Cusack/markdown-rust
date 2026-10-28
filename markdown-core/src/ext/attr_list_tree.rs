//! Port of `markdown/extensions/attr_list.py` (`attr_list`
//! treeprocessor, priority 8).
//!
//! Pre-order walk mirroring `doc.iter()`; only text/tail/attr edits, no
//! structural changes, so index paths stay valid.

use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};
use crate::patterns_gen as P;

fn header_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_ATTR_LIST_ATTRLISTTREEPROCESSOR_HEADER_RE)
}
fn block_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_ATTR_LIST_ATTRLISTTREEPROCESSOR_BLOCK_RE)
}
fn inline_re() -> fancy_regex::Regex {
    compile(P::RE_EXT_ATTR_LIST_ATTRLISTTREEPROCESSOR_INLINE_RE)
}

fn is_header(tag: &str) -> bool {
    matches!(tag, "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
}

fn assign_attrs(el: &mut Element, attrs_string: &str, strict: bool) -> String {
    let (attrs, remainder) = super::attr_list::get_attrs_and_remainder(attrs_string);
    if strict && !remainder.is_empty() {
        return remainder;
    }
    for (k, v) in &attrs {
        if k == "." {
            let cls = el.get_attr("class").unwrap_or("").to_string();
            if cls.is_empty() {
                el.set_attr("class", v);
            } else {
                el.set_attr("class", &format!("{cls} {v}"));
            }
        } else {
            el.set_attr(&super::attr_list::sanitize_name(k), v);
        }
    }
    remainder
}

fn nav<'x>(root: &'x Element, path: &[usize]) -> &'x Element {
    let mut node = root;
    for &i in path {
        node = &node.children[i];
    }
    node
}

fn nav_mut<'x>(root: &'x mut Element, path: &[usize]) -> &'x mut Element {
    let mut node = root;
    for &i in path {
        node = &mut node.children[i];
    }
    node
}

/// `AttrListTreeprocessor.run`.
pub fn run_tree(ctx: &Context, root: &mut Element) {
    fn walk(ctx: &Context, root: &mut Element, path: &[usize]) {
        process_one(ctx, root, path);
        let n = nav(root, path).children.len();
        for i in 0..n {
            let mut sub = path.to_vec();
            sub.push(i);
            walk(ctx, root, &sub);
        }
    }
    walk(ctx, root, &[]);
}

fn process_one(ctx: &Context, root: &mut Element, path: &[usize]) {
    let tag = nav(root, path).tag.clone();
    if !ctx.is_block_level(&tag) {
        if path.is_empty() {
            return;
        }
        // Inline: check for attrs at start of tail.
        let t = nav(root, path).tail.clone();
        if let Some(t) = t {
            if let Ok(Some(m)) = inline_re().find(t.as_str()) {
                if m.start() == 0 {
                    let caps = inline_re().captures(t.as_str()).unwrap().unwrap();
                    let inner = caps.get(1).unwrap().as_str().to_string();
                    let end = m.end();
                    let remainder = {
                        let parent_path = &path[..path.len() - 1];
                        let idx = path[path.len() - 1];
                        assign_attrs(&mut nav_mut(root, parent_path).children[idx], &inner, false)
                    };
                    let tail_text = t.into_string();
                    let new_tail = format!("{}{}", &tail_text[end..], remainder);
                    let parent_path = &path[..path.len() - 1];
                    let idx = path[path.len() - 1];
                    nav_mut(root, parent_path).children[idx].tail = Some(Text::Normal(new_tail));
                }
            }
        }
        return;
    }
    // Block level.
    let re = if is_header(&tag) || tag == "dt" || tag == "td" || tag == "th" {
        header_re()
    } else {
        block_re()
    };
    let is_li = tag == "li";
    let child_tags: Vec<String> = nav(root, path)
        .children
        .iter()
        .map(|c| c.tag.clone())
        .collect();
    if is_li && !child_tags.is_empty() {
        let mut pos: Option<usize> = None;
        for (i, t) in child_tags.iter().enumerate() {
            if t == "ul" || t == "ol" {
                pos = Some(i);
                break;
            }
        }
        match pos {
            None => {
                let t = nav(root, path).children.last().and_then(|c| c.tail.clone());
                if let Some(t) = t {
                    if let Ok(Some(m)) = re.find(t.as_str()) {
                        let caps = re.captures(t.as_str()).unwrap().unwrap();
                        let inner = caps.get(1).unwrap().as_str().to_string();
                        let start = m.start();
                        let remainder = assign_attrs(nav_mut(root, path), &inner, true);
                        if remainder.is_empty() {
                            let el = nav_mut(root, path);
                            let last = el.children.len() - 1;
                            let tail = el.children[last].tail.take().unwrap().into_string();
                            el.children[last].tail =
                                Some(Text::Normal(tail[..start].to_string()));
                        }
                    }
                }
            }
            Some(p) if p > 0 => {
                let t = nav(root, path).children[p - 1].tail.clone();
                if let Some(t) = t {
                    if let Ok(Some(m)) = re.find(t.as_str()) {
                        let caps = re.captures(t.as_str()).unwrap().unwrap();
                        let inner = caps.get(1).unwrap().as_str().to_string();
                        let start = m.start();
                        let remainder = assign_attrs(nav_mut(root, path), &inner, true);
                        if remainder.is_empty() {
                            let el = nav_mut(root, path);
                            let tail = el.children[p - 1].tail.take().unwrap().into_string();
                            el.children[p - 1].tail =
                                Some(Text::Normal(tail[..start].to_string()));
                        }
                    }
                }
            }
            _ => {
                let t = nav(root, path).text.clone();
                if let Some(t) = t {
                    if let Ok(Some(m)) = re.find(t.as_str()) {
                        let caps = re.captures(t.as_str()).unwrap().unwrap();
                        let inner = caps.get(1).unwrap().as_str().to_string();
                        let start = m.start();
                        let remainder = assign_attrs(nav_mut(root, path), &inner, true);
                        if remainder.is_empty() {
                            let el = nav_mut(root, path);
                            let text = el.text.take().unwrap().into_string();
                            el.text = Some(Text::Normal(text[..start].to_string()));
                        }
                    }
                }
            }
        }
        return;
    }
    let has_children = !nav(root, path).children.is_empty();
    if has_children {
        let t = nav(root, path)
            .children
            .last()
            .and_then(|c| c.tail.clone());
        if let Some(t) = t {
            if let Ok(Some(m)) = re.find(t.as_str()) {
                let caps = re.captures(t.as_str()).unwrap().unwrap();
                let inner = caps.get(1).unwrap().as_str().to_string();
                let start = m.start();
                let remainder = assign_attrs(nav_mut(root, path), &inner, true);
                if remainder.is_empty() {
                    let el = nav_mut(root, path);
                    let last = el.children.len() - 1;
                    let tail = el.children[last].tail.take().unwrap().into_string();
                    el.children[last].tail =
                        Some(Text::Normal(tail[..start].to_string()));
                    if is_header(&tag) {
                        let el = nav_mut(root, path);
                        let last = el.children.len() - 1;
                        let t2 = el.children[last].tail.take().unwrap().into_string();
                        el.children[last].tail = Some(Text::Normal(
                            t2.trim_end_matches('#').trim_end().to_string(),
                        ));
                    }
                }
            }
        }
    } else {
        let t = nav(root, path).text.clone();
        if let Some(t) = t {
            if let Ok(Some(m)) = re.find(t.as_str()) {
                let caps = re.captures(t.as_str()).unwrap().unwrap();
                let inner = caps.get(1).unwrap().as_str().to_string();
                let start = m.start();
                let remainder = assign_attrs(nav_mut(root, path), &inner, true);
                if remainder.is_empty() {
                    let el = nav_mut(root, path);
                    let text = el.text.take().unwrap().into_string();
                    el.text = Some(Text::Normal(text[..start].to_string()));
                    if is_header(&tag) {
                        let t2 = el.text.take().unwrap().into_string();
                        el.text = Some(Text::Normal(
                            t2.trim_end_matches('#').trim_end().to_string(),
                        ));
                    }
                }
            }
        }
    }
}
