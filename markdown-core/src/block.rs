//! `BlockParser` + `BlockProcessor`s (`blockparser.py`, `blockprocessors.py`).
//!
//! Core processor chain (upstream priorities): empty 100, indent 90,
//! code 80, hashheader 70, setextheader 60, hr 50, olist 40, ulist 30,
//! quote 20, reference 15, paragraph 10. Extension processors slot in by
//! priority when `ext/` lands (`ext::block_chain`); unknown specs decline
//! in `pipeline` before this is reached.
//!
//! Recursion accounting: every nested `parse_chunk`/`parse_blocks` bumps
//! `ctx.block_depth`. A conservative frame model declines absurd nesting
//! to the pure fallback (which raises exactly like the oracle); the
//! blockquote processor additionally replicates the exact
//! `nearing_recursion_limit` test (`context::blockquote_allowed`).

use fancy_regex::Regex as F;

use crate::consts::compile;
use crate::context::Context;
use crate::model::{Element, Text};


/// Public entry: `parseDocument` — root `div` over the joined lines.
pub fn parse_document(ctx: &mut Context, lines: &[String]) -> Element {
    ctx.block_chain = build_chain(ctx);
    let mut root = Element::new("div");
    ctx.block_depth = 0;
    parse_chunk(ctx, &mut root, &lines.join("\n"));
    root
}

/// Block processor identity in registry order (priority high to low).
/// Extension variants slot in by priority; ties keep registration order
/// (stable sort), mirroring `util.Registry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    Admonition,
    MarkdownBlock,
    Empty,
    Indent,
    DefIndent,
    Code,
    Table,
    HashHeader,
    SetextHeader,
    Hr,
    OList,
    UList,
    DefList,
    Quote,
    Footnote,
    Abbr,
    Reference,
    Paragraph,
}

fn priority(k: BlockKind) -> u32 {
    match k {
        BlockKind::Admonition => 105,
        BlockKind::MarkdownBlock => 105,
        BlockKind::Empty => 100,
        BlockKind::Indent => 90,
        BlockKind::DefIndent => 85,
        BlockKind::Code => 80,
        BlockKind::Table => 75,
        BlockKind::HashHeader => 70,
        BlockKind::SetextHeader => 60,
        BlockKind::Hr => 50,
        BlockKind::OList => 40,
        BlockKind::UList => 30,
        BlockKind::DefList => 25,
        BlockKind::Quote => 20,
        BlockKind::Footnote => 17,
        BlockKind::Abbr => 16,
        BlockKind::Reference => 15,
        BlockKind::Paragraph => 10,
    }
}

/// Registration order: defaults in `build_block_parser` order, then one
/// entry per extension spec in order (re-registrations replace in place
/// at the end, like `Registry.register`). Stable-sort by priority desc.
fn build_chain(ctx: &Context) -> Vec<BlockKind> {
    use BlockKind::*;
    let mut chain = vec![
        Empty, Indent, Code, HashHeader, SetextHeader, Hr, OList, UList, Quote, Reference,
        Paragraph,
    ];
    for s in &ctx.extensions {
        match s.name.as_str() {
            "admonition" => chain.push(Admonition),
            "md_in_html" => chain.push(MarkdownBlock),
            "def_list" => {
                chain.push(DefIndent);
                chain.push(DefList);
            }
            "tables" => chain.push(Table),
            "footnotes" => chain.push(Footnote),
            "abbr" => chain.push(Abbr),
            _ => {}
        }
    }
    // Sane lists *replace* olist/ulist (same names, new behavior).
    let sane = ctx.extensions.iter().any(|e| e.name == "sane_lists");
    let _ = sane;
    chain.sort_by_key(|a| std::cmp::Reverse(priority(*a)));
    chain
}

pub(crate) fn parse_chunk(ctx: &mut Context, parent: &mut Element, text: &str) {
    // Depth is accounted in `parse_blocks` (every nested call).
    let blocks: Vec<String> = text.split("\n\n").map(str::to_string).collect();
    parse_blocks(ctx, parent, blocks);
}

/// Trigger-byte masks for the block-processor prefilter (u32; low
/// 14 bits match the inline `T_*` assignments).
const B_BANG: u32 = 1 << 8;
const B_LBRACK: u32 = 1 << 2;
const B_STAR: u32 = 1 << 6;
const B_US: u32 = 1 << 7;
const B_HASH: u32 = 1 << 16;
const B_PIPE: u32 = 1 << 14;
const B_EQ: u32 = 1 << 15;
const B_DASH: u32 = 1 << 9;
const B_GT: u32 = 1 << 13;

/// Byte-presence mask of trigger bytes in a block.
fn block_trigger_mask(block: &str) -> u32 {
    let mut m = 0u32;
    for b in block.bytes() {
        match b {
            b'!' => m |= B_BANG,
            b'[' => m |= B_LBRACK,
            b'*' => m |= B_STAR,
            b'_' => m |= B_US,
            b'#' => m |= B_HASH,
            b'|' => m |= B_PIPE,
            b'=' => m |= B_EQ,
            b'-' => m |= B_DASH,
            b'>' => m |= B_GT,
            _ => {}
        }
        if m == (B_BANG | B_LBRACK | B_STAR | B_US | B_HASH | B_PIPE | B_EQ | B_DASH | B_GT) {
            break;
        }
    }
    m
}

/// Whether `kind`'s runner can match `block`: every runner's test
/// requires its trigger byte(s) (or, for lists, a marker/space first
/// byte), so absence is a sound skip.
fn block_may_match(kind: BlockKind, mask: u32, first: u8, block: &str, parent: &Element) -> bool {
    match kind {
        // Fresh `!!!` needs `!`; a continuation needs the live
        // admonition div as last child (exact: `parse_content` finds
        // no sibling otherwise).
        BlockKind::Admonition => {
            mask & B_BANG != 0
                || parent
                    .children
                    .last()
                    .map(|c| {
                        c.tag == "div"
                            && c.get_attr("class").map(|s| s.contains("admonition")).unwrap_or(false)
                    })
                    .unwrap_or(false)
        }
        BlockKind::MarkdownBlock => block.starts_with("\u{2}wzxhzdk:"),
        BlockKind::Empty
        | BlockKind::Indent
        | BlockKind::Code
        | BlockKind::DefIndent
        | BlockKind::DefList
        | BlockKind::Paragraph => true,
        BlockKind::Table => mask & B_PIPE != 0,
        BlockKind::HashHeader => mask & B_HASH != 0,
        BlockKind::SetextHeader => mask & (B_EQ | B_DASH) != 0,
        BlockKind::Hr => mask & (B_STAR | B_DASH | B_US) != 0,
        BlockKind::OList => first == b' ' || first.is_ascii_digit(),
        BlockKind::UList => first == b' ' || first == b'*' || first == b'+' || first == b'-',
        BlockKind::Quote => mask & B_GT != 0,
        BlockKind::Footnote => mask & B_LBRACK != 0,
        BlockKind::Abbr => mask & B_STAR != 0,
        BlockKind::Reference => mask & B_LBRACK != 0,
    }
}

pub(crate) fn parse_blocks(ctx: &mut Context, parent: &mut Element, mut blocks: Vec<String>) {
    // Depth accounting for the recursion model (see `blockquote_allowed`):
    // every nested `parse_blocks` call is ~5 Python frames (conservative:
    // declining early just takes the pure fallback, which matches by
    // construction; the blockquote test uses the exact k=3 formula).
    ctx.block_depth += 1;
    let depth_ok = ctx
        .recursion_limit
        .saturating_sub(ctx.py_entry_depth + 5 + 3 * ctx.block_depth)
        >= 100;
    if !depth_ok {
        ctx.block_depth -= 1;
        ctx.declined = true;
        return;
    }
    while !blocks.is_empty() && !ctx.declined {
        let chain = ctx.block_chain.clone();
        // Trigger prefilter (P6-adjacent): skip processors whose match
        // requires a byte the block lacks. Masks are necessary
        // conditions of each runner's test, so skipping is exact.
        let bmask = block_trigger_mask(&blocks[0]);
        let b0 = blocks[0].as_bytes().first().copied().unwrap_or(0);
        for kind in chain {
            if !block_may_match(kind, bmask, b0, &blocks[0], parent) {
                continue;
            }
            let hit = match kind {
                BlockKind::Admonition => crate::ext::admonition::run(ctx, parent, &mut blocks),
                BlockKind::MarkdownBlock => {
                    crate::ext::md_in_html::run_markdown_block(ctx, parent, &mut blocks)
                }
                BlockKind::Empty => run_empty(ctx, parent, &mut blocks),
                BlockKind::Indent => run_indent(ctx, parent, &mut blocks),
                BlockKind::DefIndent => run_indent(ctx, parent, &mut blocks),
                BlockKind::Code => run_code(ctx, parent, &mut blocks),
                BlockKind::Table => crate::ext::tables::run(ctx, parent, &mut blocks),
                BlockKind::HashHeader => run_hashheader(ctx, parent, &mut blocks),
                BlockKind::SetextHeader => run_setextheader(ctx, parent, &mut blocks),
                BlockKind::Hr => run_hr(ctx, parent, &mut blocks),
                BlockKind::OList => run_olist(ctx, parent, &mut blocks, false),
                BlockKind::UList => run_ulist(ctx, parent, &mut blocks),
                BlockKind::DefList => crate::ext::def_list::run(ctx, parent, &mut blocks),
                BlockKind::Quote => run_quote(ctx, parent, &mut blocks),
                BlockKind::Footnote => crate::ext::footnotes::run_block(ctx, parent, &mut blocks),
                BlockKind::Abbr => crate::ext::abbr::run_block(ctx, parent, &mut blocks),
                BlockKind::Reference => run_reference(ctx, parent, &mut blocks),
                BlockKind::Paragraph => {
                    run_paragraph(ctx, parent, &mut blocks);
                    true
                }
            };
            if hit {
                break;
            }
        }
    }
    ctx.block_depth -= 1;
}


fn last_child_tag(parent: &Element) -> Option<&str> {
    parent.children.last().map(|c| c.tag.as_str())
}

fn tab_patterns(tab: usize) -> (F, F, F, F, F) {
    let indent = compile(&format!("^(([ ]{{{tab}}})+)"));
    let listn = tab.saturating_sub(1);
    let ol = compile(&format!(r"^[ ]{{0,{listn}}}\d+\.[ ]+(.*)"));
    let child = compile(&format!(r"^[ ]{{0,{listn}}}((\d+\.)|[*+-])[ ]+(.*)"));
    let ind = compile(&format!(r"^[ ]{{{tab},{}}}((\d+\.)|[*+-])[ ]+.*", 2 * tab - 1));
    let ul = compile(&format!(r"^[ ]{{0,{listn}}}[*+-][ ]+(.*)"));
    (indent, ol, child, ind, ul)
}


/// Returns (test_re, child_re, sibling_tags, lazy_ol) for the call.
fn list_cfg(
    ctx: &Context,
    tab: usize,
    ordered: bool,
) -> (F, F, &'static [&'static str], bool) {
    let sane = ctx.extensions.iter().any(|e| e.name == "sane_lists");
    let listn = tab.saturating_sub(1);
    if !sane {
        if ordered {
            let (_, ol, child, _, _) = tab_patterns(tab);
            (ol, child, &["ol", "ul"], true)
        } else {
            let (_, _, child, _, ul) = tab_patterns(tab);
            (ul, child, &["ol", "ul"], true)
        }
    } else if ordered {
        let (_, ol, _, _, _) = tab_patterns(tab);
        (
            ol,
            compile(&format!("^[ ]{{0,{listn}}}((\\d+)\\.)[ ]+(.*)")),
            &["ol"],
            false,
        )
    } else {
        let (_, _, _, _, ul) = tab_patterns(tab);
        (
            ul,
            compile(&format!("^[ ]{{0,{listn}}}(([*+-]))[ ]+(.*)")),
            &["ul"],
            true,
        )
    }
}

/// `BlockProcessor.detab`.
pub(crate) fn detab(text: &str, length: usize) -> (String, String) {
    let mut kept = Vec::new();
    let lines: Vec<&str> = text.split('\n').collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with(&" ".repeat(length)) {
            kept.push(line[length..].to_string());
        } else if line.trim().is_empty() {
            kept.push(String::new());
        } else {
            break;
        }
        i += 1;
    }
    (kept.join("\n"), lines[i..].join("\n"))
}

/// `BlockProcessor.looseDetab`.
fn loose_detab(text: &str, tab_length: usize, level: usize) -> String {
    let pad = " ".repeat(tab_length * level);
    text.split('\n')
        .map(|line| line.strip_prefix(&pad).unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn indent_re(tab: usize) -> F {
    compile(&format!("^(([ ]{{{tab}}})+)"))
}

struct IndentCfg<'x> {
    items: &'x [&'x str],
    lists: &'x [&'x str],
    create_tag: &'x str,
}

fn indent_cfg(ctx: &Context) -> IndentCfg<'static> {
    if ctx.extensions.iter().any(|e| e.name == "def_list") {
        IndentCfg { items: &["dd", "li"], lists: &["dl", "ol", "ul"], create_tag: "dd" }
    } else {
        IndentCfg { items: &["li"], lists: &["ul", "ol"], create_tag: "li" }
    }
}

// ---- EmptyBlockProcessor (100) ----

fn run_empty(
    _ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    if !blocks[0].is_empty() && !blocks[0].starts_with('\n') {
        return false;
    }
    let block = blocks.remove(0);
    let mut filler = "\n\n".to_string();
    if !block.is_empty() {
        filler = "\n".to_string();
        let rest = block[1..].to_string();
        if !rest.is_empty() {
            blocks.insert(0, rest);
        }
    }
    if let Some(sib) = parent.children.last_mut() {
        if sib.tag == "pre" && !sib.children.is_empty() && sib.children[0].tag == "code" {
            let code = &mut sib.children[0];
            let cur = code.text.take().map(|t| t.into_string()).unwrap_or_default();
            code.text = Some(Text::Atomic(format!("{cur}{filler}")));
        }
    }
    true
}

fn run_indent(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    let tab = ctx.tab_length;
    let block = &blocks[0];
    if !block.starts_with(&" ".repeat(tab)) || ctx.bst_is("detabbed") {
        return false;
    }
    let cfg = indent_cfg(ctx);
    let parent_is_item = cfg.items.contains(&parent.tag.as_str());
    let last_is_list = matches!(last_child_tag(parent), Some(t) if cfg.lists.contains(&t));
    if !(parent_is_item || last_is_list) {
        return false;
    }
    let block = blocks.remove(0);
    // `get_level`: walk down list/item children (immutable index path).
    let mut indent_level = 0usize;
    if let Ok(Some(m)) = indent_re(tab).find(&block) {
        if m.start() == 0 {
            indent_level = m.as_str().len() / tab;
        }
    }
    let mut level = if ctx.bst_is("list") { 1 } else { 0 };
    let mut path: Vec<usize> = Vec::new();
    loop {
        if indent_level <= level {
            break;
        }
        let mut node = &*parent;
        for &i in &path {
            node = &node.children[i];
        }
        match node.children.last().map(|c| c.tag.as_str()) {
            Some(t) if cfg.lists.contains(&t) => {
                level += 1;
                path.push(node.children.len() - 1);
            }
            Some(t) if cfg.items.contains(&t) => {
                path.push(node.children.len() - 1);
            }
            _ => break,
        }
        if path.len() > 1024 {
            break;
        }
    }
    let block = loose_detab(&block, tab, level);
    let cfg = indent_cfg(ctx);
    ctx.bst_set("detabbed");
    if cfg.items.contains(&parent.tag.as_str()) {
        // The parent is already a list item. Just parse the child block.
        let last_is_list = parent
            .children
            .last()
            .map(|c| cfg.lists.contains(&c.tag.as_str()))
            .unwrap_or(false);
        if last_is_list {
            let idx = parent.children.len() - 1;
            parse_blocks(ctx, &mut parent.children[idx], vec![block]);
        } else {
            parse_blocks(ctx, parent, vec![block]);
        }
    } else {
        // Branches key on the walked-to node (`sibling` upstream).
        let mut node: &mut Element = parent;
        for &i in &path {
            node = &mut node.children[i];
        }
        if cfg.items.contains(&node.tag.as_str()) {
            parse_blocks(ctx, node, vec![block]);
        } else if node.children.last().map(|c| cfg.items.contains(&c.tag.as_str())).unwrap_or(false) {
            let idx = node.children.len() - 1;
            let li = &mut node.children[idx];
            let nonempty = matches!(&li.text, Some(t) if !t.as_str().is_empty());
            if nonempty {
                let s = li.text.take().unwrap().into_string();
                let mut p = Element::new("p");
                p.text = Some(Text::Normal(s));
                li.text = Some(Text::Normal(String::new()));
                li.children.insert(0, p);
            }
            // `parseChunk(sibling[-1], block)`.
            let chunks: Vec<String> = block.split("\n\n").map(str::to_string).collect();
            for chunk in chunks {
                parse_blocks(ctx, li, vec![chunk]);
            }
        } else {
            // `create_item(sibling, block)`.
            node.children.push(Element::new(cfg.create_tag));
            let idx = node.children.len() - 1;
            parse_blocks(ctx, &mut node.children[idx], vec![block]);
        }
    }
    ctx.bst_reset();
    true
}
// ---- CodeBlockProcessor (80) ----

fn run_code(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    let tab = ctx.tab_length;
    if !blocks[0].starts_with(&" ".repeat(tab)) {
        return false;
    }
    let block = blocks.remove(0);
    let append_to_prev = parent
        .children
        .last()
        .map(|s| {
            s.tag == "pre" && !s.children.is_empty() && s.children[0].tag == "code"
        })
        .unwrap_or(false);
    if append_to_prev {
        let code = &mut parent.children.last_mut().unwrap().children[0];
        let (detabbed, rest) = detab(&block, tab);
        let cur = code.text.take().map(|t| t.into_string()).unwrap_or_default();
        code.text = Some(Text::Atomic(format!(
            "{cur}\n{}\n",
            crate::consts::code_escape(detabbed.trim_end())
        )));
        if !rest.is_empty() {
            blocks.insert(0, rest);
        }
    } else {
        let (detabbed, rest) = detab(&block, tab);
        let mut pre = Element::new("pre");
        let mut code = Element::new("code");
        code.text = Some(Text::Atomic(format!(
            "{}\n",
            crate::consts::code_escape(detabbed.trim_end())
        )));
        pre.children.push(code);
        parent.children.push(pre);
        if !rest.is_empty() {
            blocks.insert(0, rest);
        }
    }
    true
}

// ---- BlockQuoteProcessor (20) ----

fn run_quote(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    use crate::patterns_gen as P;
    let re = compile(P::RE_BLK_BLOCKQUOTEPROCESSOR_RE);
    let hit = match re.find(&blocks[0]) {
        Ok(m) => m,
        Err(_) => return false,
    };
    let hit = hit.is_some();
    if !hit || !ctx.blockquote_allowed() {
        return false;
    }
    let block = blocks.remove(0);
    let m = re.find(&block).unwrap().unwrap();
    let before = block[..m.start()].to_string();
    // Lines before the blockquote parse first.
    parse_blocks(ctx, parent, vec![before]);
    // Remove `> ` from the beginning of each line.
    let cleaned: Vec<String> = block[m.start()..]
        .split('\n')
        .map(|line| bq_clean(&re, line))
        .collect();
    let block = cleaned.join("\n");
    // Sibling blockquote reuse.
    let sib_is_quote = parent
        .children
        .last()
        .map(|s| s.tag == "blockquote")
        .unwrap_or(false);
    if !sib_is_quote {
        parent.children.push(Element::new("blockquote"));
    }
    let idx = parent.children.len() - 1;
    ctx.bst_set("blockquote");
    let chunks: Vec<String> = block.split("\n\n").map(str::to_string).collect();
    for chunk in chunks {
        parse_blocks(ctx, &mut parent.children[idx], vec![chunk]);
    }
    ctx.bst_reset();
    true
}

fn bq_clean(re: &F, line: &str) -> String {
    // `RE.match(line)`: anchored at 0 via suffix slice.
    let m = re.find(line).ok().flatten().filter(|m| m.start() == 0);
    if line.trim() == ">" {
        return String::new();
    }
    match m {
        Some(m) => {
            // group(2): rest after `> ?`.
            let caps = re.captures(line).unwrap().unwrap();
            let g2 = caps.get(2).map(|g| g.as_str()).unwrap_or("");
            let _ = m;
            g2.to_string()
        }
        None => line.to_string(),
    }
}
// ---- OListProcessor (40) / UListProcessor (30) ----

fn run_olist(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
    _ordered: bool,
) -> bool {
    run_list(ctx, parent, blocks, true)
}

fn run_ulist(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    run_list(ctx, parent, blocks, false)
}

fn run_list(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
    ordered: bool,
) -> bool {
    let tab = ctx.tab_length;
    let (_, _, _, indent_re, _) = tab_patterns(tab);
    let (item_re, child_re, sibling_tags, lazy_ol) = list_cfg(ctx, tab, ordered);
    if item_re.find(&blocks[0]).ok().flatten().filter(|m| m.start() == 0).is_none() {
        return false;
    }
    let block = blocks.remove(0);
    // `get_items`: split into items.
    let mut items: Vec<String> = Vec::new();
    let mut first_marker: Option<String> = None;
    for line in block.split('\n') {
        if child_re.find(line).ok().flatten().filter(|m| m.start() == 0).is_some() {
            let caps = child_re.captures(line).unwrap().unwrap();
            if items.is_empty() {
                let marker = caps.get(1).map(|g| g.as_str()).unwrap_or("");
                let digits: String = marker.chars().take_while(|c| c.is_ascii_digit()).collect();
                if !digits.is_empty() {
                    first_marker = Some(digits);
                }
            }
            items.push(caps.get(3).map(|g| g.as_str()).unwrap_or("").to_string());
        } else if indent_re.find(line).ok().flatten().filter(|m| m.start() == 0).is_some() {
            if items.is_empty() {
                items.push(line.to_string());
            } else if items.last().unwrap().starts_with(&" ".repeat(tab)) {
                let last = items.len() - 1;
                items[last] = format!("{}\n{line}", items[last]);
            } else {
                items.push(line.to_string());
            }
        } else if items.is_empty() {
            items.push(line.to_string());
        } else {
            let last = items.len() - 1;
            items[last] = format!("{}\n{line}", items[last]);
        }
    }
    let tag = if ordered { "ol" } else { "ul" };
    // Sibling list reuse.
    let sib_is_list = parent
        .children
        .last()
        .map(|s| sibling_tags.contains(&s.tag.as_str()))
        .unwrap_or(false);
    if sib_is_list {
        let lidx = parent.children.len() - 1;
        // Make sure the previous item's text is wrapped in a `p`.
        {
            let lst = &mut parent.children[lidx];
            let last = lst.children.len() - 1;
            let li = &mut lst.children[last];
            let nonempty = matches!(&li.text, Some(t) if !t.as_str().is_empty());
            if nonempty {
                let s = li.text.take().unwrap().into_string();
                let mut p = Element::new("p");
                p.text = Some(Text::Normal(s));
                li.text = Some(Text::Normal(String::new()));
                li.children.insert(0, p);
            }
            // Tail of the last child moves into a `p`.
            let lch_tail = li.children.last().and_then(|c| c.tail.clone());
            if let Some(t) = lch_tail {
                if !t.as_str().is_empty() {
                    let lt = t.into_string();
                    let lt = lt.trim_start().to_string();
                    let lch_idx = li.children.len() - 1;
                    li.children[lch_idx].tail = Some(Text::Normal(String::new()));
                    let mut p = Element::new("p");
                    p.text = Some(Text::Normal(lt));
                    li.children.push(p);
                }
            }
        }
        // Parse the first item in `looselist` state.
        let first = items.remove(0);
        let lst = &mut parent.children[lidx];
        lst.children.push(Element::new("li"));
        let li_idx = lst.children.len() - 1;
        ctx.bst_set("looselist");
        parse_blocks(ctx, &mut lst.children[li_idx], vec![first]);
        ctx.bst_reset();
        parse_list_items(ctx, parent, lidx, items, tab);
    } else if parent.tag == "ol" || parent.tag == "ul" {
        // Edge case: blank parent list item (see `ListIndentProcessor`).
        parse_list_items(ctx, parent, usize::MAX, items, tab);
    } else {
        parent.children.push(Element::new(tag));
        let lidx = parent.children.len() - 1;
        if ordered && !lazy_ol {
            if let Some(sw) = first_marker {
                if sw != "1" {
                    parent.children[lidx].set_attr("start", &sw);
                }
            }
        }
        parse_list_items(ctx, parent, lidx, items, tab);
    }
    true
}

/// Shared item loop: `state.set('list')`, indented items parse under the
/// last item, others get fresh `li`s. `lidx == usize::MAX` means the
/// parent itself is the list (blank-item edge case).
fn parse_list_items(
    ctx: &mut Context,
    parent: &mut Element,
    lidx: usize,
    items: Vec<String>,
    tab: usize,
) {
    let pad = " ".repeat(tab);
    let parse_under = |ctx: &mut Context, lst: &mut Element, item: String| {
        if item.starts_with(&pad) {
            let last = lst.children.len() - 1;
            parse_blocks(ctx, &mut lst.children[last], vec![item]);
        } else {
            lst.children.push(Element::new("li"));
            let li_idx = lst.children.len() - 1;
            parse_blocks(ctx, &mut lst.children[li_idx], vec![item]);
        }
    };
    ctx.bst_set("list");
    if lidx == usize::MAX {
        for item in items {
            if item.starts_with(&pad) {
                let last = parent.children.len() - 1;
                parse_blocks(ctx, &mut parent.children[last], vec![item]);
            } else {
                parent.children.push(Element::new("li"));
                let li_idx = parent.children.len() - 1;
                parse_blocks(ctx, &mut parent.children[li_idx], vec![item]);
            }
        }
    } else {
        let lst = &mut parent.children[lidx];
        for item in items {
            parse_under(ctx, lst, item);
        }
    }
    ctx.bst_reset();
}
// ---- HashHeaderProcessor (70) ----

fn run_hashheader(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    use crate::patterns_gen as P;
    let re = compile(P::RE_BLK_HASHHEADERPROCESSOR_RE);
    if re.find(&blocks[0]).ok().flatten().is_none() {
        return false;
    }
    let block = blocks.remove(0);
    let m = re.find(&block).unwrap().unwrap();
    let caps = re.captures(&block).unwrap().unwrap();
    let before = block[..m.start()].to_string();
    let after = block[m.end()..].to_string();
    if !before.is_empty() {
        parse_blocks(ctx, parent, vec![before]);
    }
    let level = caps.get(1).map(|g| g.as_str().len()).unwrap_or(1);
    let header = caps.get(2).map(|g| g.as_str().trim()).unwrap_or("");
    let mut h = Element::new(&format!("h{level}"));
    h.text = Some(Text::Normal(header.to_string()));
    parent.children.push(h);
    if !after.is_empty() {
        let after = if ctx.bst_is("looselist") {
            loose_detab(&after, ctx.tab_length, 1)
        } else {
            after
        };
        blocks.insert(0, after);
    }
    true
}

// ---- SetextHeaderProcessor (60) ----

fn run_setextheader(
    _ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    use crate::patterns_gen as P;
    let re = compile(P::RE_BLK_SETEXTHEADERPROCESSOR_RE);
    // `RE.match(block)`: anchored at 0.
    let is_match = re
        .find(&blocks[0])
        .ok()
        .flatten()
        .map(|m| m.start() == 0)
        .unwrap_or(false);
    if !is_match {
        return false;
    }
    let block = blocks.remove(0);
    let lines: Vec<&str> = block.split('\n').collect();
    let level = if lines[1].starts_with('=') { 1 } else { 2 };
    let mut h = Element::new(&format!("h{level}"));
    h.text = Some(Text::Normal(lines[0].trim().to_string()));
    parent.children.push(h);
    if lines.len() > 2 {
        blocks.insert(0, lines[2..].join("\n"));
    }
    true
}

// ---- HRProcessor (50) ----

fn run_hr(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    use crate::patterns_gen as P;
    let re = compile(P::RE_BLK_HRPROCESSOR_SEARCH_RE);
    let m = match re.find(&blocks[0]) {
        Ok(Some(m)) => m,
        _ => return false,
    };
    let (ms, me) = (m.start(), m.end());
    let block = blocks.remove(0);
    let prelines = block[..ms].trim_end_matches('\n').to_string();
    if !prelines.is_empty() {
        parse_blocks(ctx, parent, vec![prelines]);
    }
    parent.children.push(Element::new("hr"));
    let postlines = block[me..].trim_start_matches('\n').to_string();
    if !postlines.is_empty() {
        blocks.insert(0, postlines);
    }
    true
}

// ---- ReferenceProcessor (15) ----

fn run_reference(
    ctx: &mut Context,
    _parent: &mut Element,
    blocks: &mut Vec<String>,
) -> bool {
    use crate::patterns_gen as P;
    let re = compile(P::RE_BLK_REFERENCEPROCESSOR_RE);
    let block = blocks.remove(0);
    let (ms, me, groups) = match re.find(&block) {
        Ok(Some(m)) => {
            let caps = re.captures(&block).unwrap().unwrap();
            let g = |i: usize| caps.get(i).map(|g| g.as_str().to_string());
            (m.start(), m.end(), (g(1), g(2), g(5), g(6)))
        }
        _ => {
            blocks.insert(0, block);
            return false;
        }
    };
    let id = groups.0.unwrap_or_default().trim().to_lowercase();
    let mut link = groups.1.unwrap_or_default();
    // Upstream: `link.lstrip('<').rstrip('>')` — independent strips.
    link = link.trim_start_matches('<').trim_end_matches('>').to_string();
    let title = groups.2.or(groups.3);
    ctx.references.insert(id, (link, title));
    if !block[me..].trim().is_empty() {
        blocks.insert(0, block[me..].trim_start_matches('\n').to_string());
    }
    if !block[..ms].trim().is_empty() {
        blocks.insert(0, block[..ms].trim_end_matches('\n').to_string());
    }
    true
}

// ---- ParagraphProcessor (10) ----

fn run_paragraph(
    ctx: &mut Context,
    parent: &mut Element,
    blocks: &mut Vec<String>,
) {
    let block = blocks.remove(0);
    if block.trim().is_empty() {
        return;
    }
    if ctx.bst_is("list") {
        match parent.children.last_mut() {
            Some(sib) => match &mut sib.tail {
                Some(Text::Normal(t)) => {
                    t.push('\n');
                    t.push_str(&block);
                }
                Some(Text::Atomic(t)) => {
                    t.push('\n');
                    t.push_str(&block);
                }
                None => {
                    sib.tail = Some(Text::Normal(format!("\n{block}")));
                }
            },
            None => match &mut parent.text {
                Some(Text::Normal(t)) => {
                    t.push('\n');
                    t.push_str(&block);
                }
                Some(Text::Atomic(t)) => {
                    t.push('\n');
                    t.push_str(&block);
                }
                None => {
                    parent.text = Some(Text::Normal(block.trim_start().to_string()));
                }
            },
        }
    } else {
        let mut p = Element::new("p");
        p.text = Some(Text::Normal(block.trim_start().to_string()));
        parent.children.push(p);
    }
}

