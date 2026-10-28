//! Port of `markdown/extensions/footnotes.py`.
//!
//! Block processor (17) harvests definitions; inline pattern (175, wired
//! in `inline.rs`) links references; tree processors build (50), reorder
//! (19, only when `USE_DEFINITION_ORDER` is false) and duplicate-backlink
//! (15) the footnote div; postprocessor (25) restores link text.
//!
//! Per-render state lives on `Context` (fresh per render, mirroring
//! `Extension.reset()`); only `unique_prefix` carries across renders and
//! is read off the real extension instance by the facade.

use crate::context::Context;
use crate::model::{Element, Text};
use crate::RenderError;

pub const FN_BACKLINK_TEXT: &str = "\u{2}zz1337820767766393qq\u{3}";
pub const NBSP_PLACEHOLDER: &str = "\u{2}qq3936677670287331zz\u{3}";

fn block_re() -> fancy_regex::Regex {
    crate::consts::compile(crate::patterns_gen::RE_EXT_FOOTNOTES_FOOTNOTEBLOCKPROCESSOR_RE)
}

fn separator(ctx: &Context) -> String {
    ctx.ext_str("footnotes", "SEPARATOR", ":")
}

// ---- state helpers (mirror FootnoteExtension methods) ----

pub(crate) fn set_footnote(ctx: &mut Context, id: &str, text: &str) {
    if let Some(slot) = ctx.footnotes.iter_mut().find(|(k, _)| k == id) {
        slot.1 = text.to_string();
    } else {
        ctx.footnotes.push((id.to_string(), text.to_string()));
    }
}

pub(crate) fn add_footnote_ref(ctx: &mut Context, id: &str) {
    if !ctx.footnote_order.iter().any(|x| x == id) {
        ctx.footnote_order.push(id.to_string());
    }
}

pub(crate) fn unique_ref(
    ctx: &mut Context,
    reference: &str,
    found: bool,
) -> Result<String, RenderError> {
    if !found {
        return Ok(reference.to_string());
    }
    let sep = separator(ctx);
    let mut reference = reference.to_string();
    let original = reference.clone();
    while ctx.used_refs.contains(&reference) {
        let (r, rest) = reference.split_once(&sep).ok_or_else(|| {
            RenderError::Unsupported("footnote unique_ref split".to_string())
        })?;
        let mut new_ref = None;
        if let Some(stripped) = r.strip_prefix("fnref") {
            let digits: String = stripped.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                let n: u64 = digits.parse().unwrap_or(1);
                new_ref = Some(format!("fnref{}{}{}", n + 1, sep, rest));
            }
        }
        reference = match new_ref {
            Some(s) => s,
            None => format!("{r}2{sep}{rest}"),
        };
    }
    ctx.used_refs.insert(reference.clone());
    *ctx.found_refs.entry(original).or_insert(0) += 1;
    Ok(reference)
}

pub(crate) fn make_footnote_id(ctx: &Context, id: &str) -> String {
    let sep = separator(ctx);
    if ctx.ext_bool("footnotes", "UNIQUE_IDS", false) {
        format!("fn{sep}{}{sep}{id}", ctx.unique_prefix)
    } else {
        format!("fn{sep}{id}")
    }
}

pub(crate) fn make_footnote_ref_id(
    ctx: &mut Context,
    id: &str,
    found: bool,
) -> Result<String, RenderError> {
    let sep = separator(ctx);
    if ctx.ext_bool("footnotes", "UNIQUE_IDS", false) {
        let prefix = ctx.unique_prefix;
        unique_ref(ctx, &format!("fnref{sep}{prefix}-{id}"), found)
    } else {
        unique_ref(ctx, &format!("fnref{sep}{id}"), found)
    }
}

pub(crate) fn backlink_title_template(ctx: &Context) -> String {
    // `__init__` does `.replace("%d", "{}")` on the configured value.
    ctx.ext_str(
        "footnotes",
        "BACKLINK_TITLE",
        "Jump back to footnote %d in the text",
    )
    .replace("%d", "{}")
}

/// Validate `{}` templates upfront (`str.format` would raise `IndexError`
/// on >1 slot; the facade declines so the pure path raises identically).
pub(crate) fn validate_templates(ctx: &Context) -> Result<(), RenderError> {
    for key in ["SUPERSCRIPT_TEXT", "BACKLINK_TITLE"] {
        let t = if key == "BACKLINK_TITLE" {
            backlink_title_template(ctx)
        } else {
            ctx.ext_str("footnotes", key, "{}")
        };
        if t.matches("{}").count() > 1 {
            return Err(RenderError::Unsupported("footnote template slots".into()));
        }
    }
    Ok(())
}

pub(crate) fn format_one(tmpl: &str, num: usize) -> String {
    if tmpl.matches("{}").count() == 1 {
        tmpl.replacen("{}", &num.to_string(), 1)
    } else {
        tmpl.to_string()
    }
}

// ---- block processor ----

/// Footnote-specific `detab`: strip one level only from indented lines.
fn fn_detab(block: &str) -> String {
    block
        .split('\n')
        .map(|line| line.strip_prefix("    ").unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn detect_tabbed(ctx: &mut Context, blocks: &mut Vec<String>) -> Vec<String> {
    let _ = ctx;
    let mut fn_blocks = Vec::new();
    let re = block_re();
    while !blocks.is_empty() && blocks[0].starts_with("    ") {
        let block = blocks.remove(0);
        match re.find(&block) {
            Ok(Some(m)) => {
                let before = block[..m.start()].trim_end_matches('\n');
                fn_blocks.push(fn_detab(before));
                blocks.insert(0, block[m.start()..].to_string());
                break;
            }
            _ => fn_blocks.push(fn_detab(&block)),
        }
    }
    fn_blocks
}

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
    let (ms, me) = (m.start(), m.end());
    let caps = re.captures(&block).unwrap().unwrap();
    let id = caps.get(1).unwrap().as_str().to_string();
    let mut fn_blocks = vec![caps.get(2).unwrap().as_str().to_string()];
    let therest = block[me..].trim_start_matches('\n');
    match re.find(therest) {
        Ok(Some(m2)) => {
            let s2 = m2.start();
            let before = therest[..s2].trim_end_matches('\n');
            fn_blocks[0] = format!("{}\n{}", fn_blocks[0], fn_detab(before))
                .trim_start_matches('\n')
                .to_string();
            blocks.insert(0, therest[s2..].to_string());
        }
        _ => {
            fn_blocks[0] = format!("{}\n{}", fn_blocks[0], fn_detab(therest))
                .trim_matches('\n')
                .to_string();
            fn_blocks.extend(detect_tabbed(ctx, blocks));
        }
    }
    let footnote = fn_blocks.join("\n\n");
    set_footnote(ctx, &id, footnote.trim_end());
    if !block[..ms].trim().is_empty() {
        blocks.insert(0, block[..ms].trim_end_matches('\n').to_string());
    }
    true
}

// ---- tree processor: build div (50) ----

/// `makeFootnotesDiv` + placeholder substitution + append.
pub fn build_div(ctx: &mut Context, root: &mut Element) -> Result<(), RenderError> {
    if ctx.footnotes.is_empty() {
        return Ok(());
    }
    let mut div = Element::new("div");
    div.set_attr("class", "footnote");
    div.children.push(Element::new("hr"));
    let mut ol = Element::new("ol");
    let mut surrogate = Element::new("div");
    let ids: Vec<String> = ctx.footnotes.iter().map(|(k, _)| k.clone()).collect();
    for (index, id) in ids.iter().enumerate() {
        let num = index + 1;
        let body = ctx
            .footnotes
            .iter()
            .find(|(k, _)| k == id)
            .map(|(_, v)| v.clone())
            .unwrap_or_default();
        let mut li = Element::new("li");
        li.set_attr("id", &make_footnote_id(ctx, id));
        crate::block::parse_chunk(ctx, &mut surrogate, &body);
        if ctx.declined {
            return Err(RenderError::Unsupported("footnote recursion".into()));
        }
        for el in surrogate.children.drain(..) {
            li.children.push(el);
        }
        let mut backlink = Element::new("a");
        let ref_id = make_footnote_ref_id(ctx, id, false)?;
        backlink.set_attr("href", &format!("#{ref_id}"));
        backlink.set_attr("class", "footnote-backref");
        backlink.set_attr("title", &format_one(&backlink_title_template(ctx), num));
        backlink.text = Some(Text::Normal(FN_BACKLINK_TEXT.to_string()));
        if !li.children.is_empty() {
            let last = li.children.len() - 1;
            if li.children[last].tag == "p" {
                let cur = li.children[last].text.take().ok_or_else(|| {
                    RenderError::Unsupported("footnote p text".into())
                })?;
                li.children[last].text = Some(Text::Normal(format!(
                    "{}{NBSP_PLACEHOLDER}",
                    cur.into_string()
                )));
                li.children[last].children.push(backlink);
            } else {
                let mut p = Element::new("p");
                p.children.push(backlink);
                li.children.push(p);
            }
        } else {
            let mut p = Element::new("p");
            p.children.push(backlink);
            li.children.push(p);
        }
        ol.children.push(li);
    }
    div.children.push(ol);
    // Placeholder substitution (mirrors findFootnotesPlaceholder).
    let marker = ctx.ext_str("footnotes", "PLACE_MARKER", "///Footnotes Go Here///");
    if let Some((child_idx, parent_path, is_text)) = find_placeholder(root, &marker) {
        if is_text {
            let node = nav_mut(root, &parent_path);
            node.children.remove(child_idx);
            node.children.insert(child_idx.min(node.children.len()), div);
        } else {
            let node = nav_mut(root, &parent_path);
            node.children[child_idx].tail = None;
            node.children.insert((child_idx + 1).min(node.children.len()), div);
        }
    } else {
        root.children.push(div);
    }
    Ok(())
}

// ---- tree helpers (index paths) ----

type Path = Vec<usize>;

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

/// Placeholder search mirroring `findFootnotesPlaceholder` (DFS).
/// Returns (child_index, parent_path, is_text).
fn find_placeholder(root: &Element, marker: &str) -> Option<(usize, Path, bool)> {
    fn finder(root: &Element, path: Path, marker: &str) -> Option<(usize, Path, bool)> {
        let node = nav(root, &path);
        for (i, child) in node.children.iter().enumerate() {
            if let Some(t) = &child.text {
                if t.as_str().contains(marker) {
                    return Some((i, path.clone(), true));
                }
            }
            if let Some(t) = &child.tail {
                if t.as_str().contains(marker) {
                    return Some((i, path.clone(), false));
                }
            }
            let mut sub = path.to_vec();
            sub.push(i);
            if let Some(r) = finder(root, sub, marker) {
                return Some(r);
            }
        }
        None
    }
    finder(root, Vec::new(), marker)
}

// ---- reorder (19) ----

/// `FootnoteReorderingProcessor.run` (only wired when
/// `USE_DEFINITION_ORDER` is false).
pub fn reorder(ctx: &mut Context, root: &mut Element) {
    if ctx.footnotes.is_empty() {
        return;
    }
    let order: Vec<String> = ctx.footnote_order.clone();
    let keys: Vec<String> = ctx.footnotes.iter().map(|(k, _)| k.clone()).collect();
    if order == keys {
        return;
    }
    // Find the footnote div.
    let div_path = find_div_by_class(root, &[], "footnote");
    let div_path = match div_path {
        Some(p) => p,
        None => return,
    };
    // old ol: the (first) `ol` child of div.
    let ol_idx = match nav(root, &div_path)
        .children
        .iter()
        .position(|c| c.tag == "ol")
    {
        Some(i) => i,
        None => return,
    };
    let mut ol_path = div_path.clone();
    ol_path.push(ol_idx);
    let mut items: Vec<Element>;
    {
        let ol = nav_mut(root, &ol_path);
        items = std::mem::take(&mut ol.children);
    }
    let sep = separator(ctx);
    let n = ctx.footnotes.len();
    items.sort_by_key(|li| {
        let id = li.get_attr("id").unwrap_or("");
        let short = id.split_once(&sep).map(|(_, r)| r).unwrap_or(id);
        order.iter().position(|x| x == short).unwrap_or(n)
    });
    // Fix backlink titles (BACKLINK_TITLE.format(index)).
    let title_tmpl = backlink_title_template(ctx);
    for (index, item) in items.iter_mut().enumerate() {
        let mut stack: Vec<*mut Element> = vec![item as *mut _];
        while let Some(ptr) = stack.pop() {
            let el = unsafe { &mut *ptr };
            if el.tag == "a" && el.get_attr("class") == Some("footnote-backref") {
                el.set_attr("title", &format_one(&title_tmpl, index + 1));
                break;
            }
            for c in el.children.iter_mut() {
                stack.push(c as *mut _);
            }
        }
    }
    {
        let parent_path = ol_path[..ol_path.len() - 1].to_vec();
        let parent = nav_mut(root, &parent_path);
        parent.children.remove(ol_idx);
        let mut new_ol = Element::new("ol");
        new_ol.children = items;
        parent.children.insert(ol_idx.min(parent.children.len()), new_ol);
    }
}

fn find_div_by_class(root: &Element, path: &[usize], class: &str) -> Option<Path> {
    let node = nav(root, path);
    for (i, child) in node.children.iter().enumerate() {
        if child.tag == "div" && child.get_attr("class") == Some(class) {
            let mut p = path.to_vec();
            p.push(i);
            return Some(p);
        }
        let mut sub = path.to_vec();
        sub.push(i);
        if let Some(r) = find_div_by_class(root, &sub, class) {
            return Some(r);
        }
    }
    None
}

// ---- duplicates (15) ----

/// `FootnotePostTreeprocessor`.
pub fn duplicates(ctx: &mut Context, root: &mut Element) {
    // Find footnote divs.
    let mut div_paths = Vec::new();
    fn collect(root: &Element, path: &[usize], out: &mut Vec<Path>) {
        let node = nav(root, path);
        for (i, child) in node.children.iter().enumerate() {
            if child.tag == "div" && child.get_attr("class") == Some("footnote") {
                let mut p = path.to_vec();
                p.push(i);
                out.push(p);
            }
            let mut sub = path.to_vec();
            sub.push(i);
            collect(root, &sub, out);
        }
    }
    collect(root, &Vec::new(), &mut div_paths);
    for dp in div_paths {
        // First `ol` under the div.
        let ol_idx = match nav(root, &dp).children.iter().position(|c| c.tag == "ol") {
            Some(i) => i,
            None => continue,
        };
        let mut olp = dp.clone();
        olp.push(ol_idx);
        let lis: Vec<Path> = {
            let ol = nav(root, &olp);
            (0..ol.children.len()).map(|i| { let mut p = olp.clone(); p.push(i); p }).collect()
        };
        let sep = separator(ctx);
        for lip in lis {
            let li_id = nav(root, &lip).get_attr("id").unwrap_or("").to_string();
            let (f, rest) = match li_id.split_once(&sep) {
                Some(t) => t,
                None => continue,
            };
            let link_id = format!("{f}ref{sep}{rest}");
            let count = ctx.found_refs.get(&link_id).copied().unwrap_or(0);
            if count <= 1 {
                continue;
            }
            // Find the backref link.
            let link_path = {
                let mut found = None;
                let mut stack = vec![lip.clone()];
                while let Some(p) = stack.pop() {
                    let el = nav(root, &p);
                    if el.tag == "a" && el.get_attr("class") == Some("footnote-backref") {
                        found = Some(p);
                        break;
                    }
                    for (i, _) in el.children.iter().enumerate() {
                        let mut sub = p.clone();
                        sub.push(i);
                        stack.push(sub);
                    }
                }
                match found {
                    Some(p) => p,
                    None => continue,
                }
            };
            let href = nav(root, &link_path).get_attr("href").unwrap_or("").to_string();
            let (r, rest2) = href.split_once(&sep).unwrap_or((href.as_str(), ""));
            let mut sibs = Vec::new();
            for index in 2..=count {
                let mut sib = nav(root, &link_path).clone();
                sib.set_attr("href", &format!("{r}{index}{sep}{rest2}"));
                sibs.push(sib);
            }
            let last_path = {
                let li = nav(root, &lip);
                let mut p = lip.clone();
                p.push(li.children.len() - 1);
                p
            };
            for s in sibs {
                nav_mut(root, &last_path).children.push(s);
            }
        }
    }
}

// ---- postprocessor (25) ----

pub fn post(ctx: &Context, text: &str) -> String {
    let backlink = ctx.ext_str("footnotes", "BACKLINK_TEXT", "&#8617;");
    text.replace(FN_BACKLINK_TEXT, &backlink)
        .replace(NBSP_PLACEHOLDER, "&#160;")
}

