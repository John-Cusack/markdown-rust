//! Inline engine (`inlinepatterns.py` + `treeprocessors.InlineProcessor`).
//!
//! Exact queue semantics (`pop(0)` BFS over the tree with `parent_map`
//! ancestors), implemented on a flat arena (`ANode` indexed by `usize`)
//! so the borrow checker stays out of the way. Placeholder numbering is
//! invisible in output (every placeholder resolves before serialization),
//! but the queue order is replicated anyway — cheap insurance.
//!
//! Byte indices throughout (`&str` slicing; all boundaries derive from
//! regex spans at char boundaries or ASCII delimiters, both safe in UTF-8).

use std::collections::{HashMap, VecDeque};

use std::sync::LazyLock;

    static BACKTICK: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_00_BACKTICKINLINEPROCESSOR));
    static ESCAPE: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_01_ESCAPEINLINEPROCESSOR));
    static REFERENCE: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_02_REFERENCEINLINEPROCESSOR));
    static LINK: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_03_LINKINLINEPROCESSOR));
    static IMAGELINK: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_04_IMAGEINLINEPROCESSOR));
    static AUTOLINK: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_08_AUTOLINKINLINEPROCESSOR));
    static AUTOMAIL: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_09_AUTOMAILINLINEPROCESSOR));
    static LINEBREAK: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_10_SUBSTITUTETAGINLINEPROCESSOR));
    static HTML: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_11_HTMLINLINEPROCESSOR));
    static ENTITY: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_12_HTMLINLINEPROCESSOR));
    static EMSTRONG: LazyLock<F> =
        LazyLock::new(|| compile(P::RE_INST_IL_13_DELIMITERPROCESSOR));
    static FOOTNOTE: LazyLock<F> = LazyLock::new(|| {
        compile(P::RE_EXTINST_FOOTNOTES_02_FOOTNOTEINLINEPROCESSOR_COMPILED_RE)
    });
    static WIKILINK: LazyLock<F> = LazyLock::new(|| {
        compile(P::RE_EXTINST_WIKILINKS_13_WIKILINKSINLINEPROCESSOR_COMPILED_RE)
    });
    static NLTAG: LazyLock<F> = LazyLock::new(|| compile("(?s)\n"));
    static HTMLSTRICT: LazyLock<F> = LazyLock::new(|| {
        let base = P::RE_INST_IL_11_HTMLINLINEPROCESSOR;
        let base = base.strip_prefix("(?s)").unwrap_or(base);
        compile(&format!("(?s){base}(?!>)"))
    });


use fancy_regex::{Captures, Regex as F};

use crate::consts::{compile, placeholder_inline, AMP_SUBSTITUTE};
use crate::context::Context;
use crate::model::{Element, Text};
use crate::patterns_gen as P;

/// Arena node: `Element` with children as indices (stable under push).
#[derive(Debug, Clone)]
struct ANode {
    tag: String,
    attrib: Vec<(String, String)>,
    text: Option<Text>,
    tail: Option<Text>,
    children: Vec<usize>,
}

struct Arena {
    nodes: Vec<ANode>,
}

impl Arena {
    fn from_element(el: &Element) -> (Self, usize) {
        let mut arena = Arena { nodes: Vec::new() };
        let root = arena.push_element(el);
        (arena, root)
    }

    fn push_element(&mut self, el: &Element) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(ANode {
            tag: el.tag.clone(),
            attrib: el.attrib.clone(),
            text: el.text.clone(),
            tail: el.tail.clone(),
            children: Vec::new(),
        });
        let kids: Vec<usize> = el.children.iter().map(|c| self.push_element(c)).collect();
        self.nodes[idx].children = kids;
        idx
    }

    fn new_node(&mut self, tag: &str) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(ANode {
            tag: tag.to_string(),
            attrib: Vec::new(),
            text: None,
            tail: None,
            children: Vec::new(),
        });
        idx
    }


    fn to_element(&self, idx: usize) -> Element {
        let n = &self.nodes[idx];
        Element {
            tag: n.tag.clone(),
            attrib: n.attrib.clone(),
            text: n.text.clone(),
            tail: n.tail.clone(),
            children: n.children.iter().map(|&c| self.to_element(c)).collect(),
        }
    }
}

#[derive(Debug, Clone)]
enum StashNode {
    El(usize),
    Str(String),
}

/// Inline patterns in registry order (priority high to low). Extension
/// patterns slot in by priority (`ext::inline_chain`); unknown specs
/// decline in `pipeline` before this is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pat {
    Backtick,
    Escape,
    Reference,
    Link,
    ImageLink,
    ImageReference,
    ShortReference,
    ShortImageReference,
    Autolink,
    Automail,
    Linebreak,
    Html,
    Entity,
    EmStrong,
    Footnote,
    Smarty(usize),
    HtmlStrict,
    WikiLink,
    Nl,
}


/// Inline pattern list with extension patterns slotted by priority.
#[allow(dead_code)]
fn patterns_for(ctx: &Context) -> Vec<Pat> {
    let mut pats: Vec<(u32, Pat)> = vec![
        (190, Pat::Backtick),
        (180, Pat::Escape),
        (170, Pat::Reference),
        (160, Pat::Link),
        (150, Pat::ImageLink),
        (140, Pat::ImageReference),
        (130, Pat::ShortReference),
        (125, Pat::ShortImageReference),
        (120, Pat::Autolink),
        (110, Pat::Automail),
        (100, Pat::Linebreak),
        (90, Pat::Html),
        (80, Pat::Entity),
        (60, Pat::EmStrong),
    ];
    let angled = ctx.has_extension("smarty")
        && ctx.ext_bool("smarty", "smart_angled_quotes", false);
    if angled {
        for p in pats.iter_mut() {
            if p.1 == Pat::Html {
                p.1 = Pat::HtmlStrict;
            }
        }
    }
    for s in &ctx.extensions {
        if s.name == "footnotes" {
            pats.push((175, Pat::Footnote));
        }
        if s.name == "wikilinks" {
            pats.push((75, Pat::WikiLink));
        }
        if s.name == "nl2br" {
            pats.push((5, Pat::Nl));
        }
    }
    pats.sort_by_key(|a| std::cmp::Reverse(a.0));
    pats.into_iter().map(|(_, p)| p).collect()
}
fn pat_regex(p: Pat) -> &'static F {
    match p {
        Pat::Backtick => &BACKTICK,
        Pat::Escape => &ESCAPE,
        Pat::Reference | Pat::ShortReference => &REFERENCE,
        Pat::Link => &LINK,
        Pat::ImageLink | Pat::ImageReference | Pat::ShortImageReference => &IMAGELINK,
        Pat::Autolink => &AUTOLINK,
        Pat::Automail => &AUTOMAIL,
        Pat::Linebreak => &LINEBREAK,
        Pat::Html => &HTML,
        Pat::Entity => &ENTITY,
        Pat::EmStrong => &EMSTRONG,
        Pat::Footnote => &FOOTNOTE,
        Pat::WikiLink => &WIKILINK,
        Pat::Nl => &NLTAG,
        Pat::HtmlStrict => &HTMLSTRICT,
        Pat::Smarty(_) => {
            unreachable!("smarty uses its own table")
        }
    }
}

#[derive(Debug, Clone)]
struct Region {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    tags: (String, String),
    size: usize,
}

#[derive(Debug, Clone)]
struct DelimEntry {
    start: usize,
    end: usize,
    is_ambiguous: bool,
    length: usize,
}

pub struct Inline<'a> {
    ctx: &'a mut Context,
    arena: Arena,
    /// Smarty-only pattern tables (empty for the main pass).
    smarty: Vec<(u32, crate::ext::smarty::SmartyPat)>,
    stashed: HashMap<usize, StashNode>,
    next_id: usize,
    // DelimiterProcessor state (per `run`, reset like upstream `reset()`).
    delim_stack: Vec<(usize, usize, bool, usize)>,
    regions: Vec<Region>,
    delim_stacks: Vec<(String, Vec<DelimEntry>)>,
    cache_index: usize,
    cache_pos: usize,
    ancestors: Vec<String>,
    parent_map: HashMap<usize, usize>,
    patterns: Vec<Pat>,
    /// Trigger-byte mask of the text currently being scanned (recomputed
    /// per `handle_inline` call); lets patterns bail without regexing.
    cur_mask: u16,
    /// Union of trigger masks of the main-pass patterns (for the
    /// whole-text early-out).
    main_union: u16,
}
impl<'a> Inline<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        let patterns = patterns_for(ctx);
        let main_union = patterns.iter().map(|p| pat_mask(*p)).fold(0, |a, m| a | m);
        Inline {
            ctx,
            arena: Arena { nodes: Vec::new() },
            stashed: HashMap::new(),
            next_id: 0,
            delim_stack: Vec::new(),
            regions: Vec::new(),
            delim_stacks: vec![("*".to_string(), Vec::new()), ("_".to_string(), Vec::new())],
            smarty: Vec::new(),
            cache_index: 0,
            cache_pos: 0,
            ancestors: Vec::new(),
            parent_map: HashMap::new(),
            patterns,
            cur_mask: 0,
            main_union,
        }
    }

    fn reset_run(&mut self) {
        self.stashed.clear();
        self.next_id = 0;
        self.delim_reset();
        self.ancestors.clear();
        self.parent_map.clear();
    }

    fn delim_reset(&mut self) {
        for (_, s) in self.delim_stacks.iter_mut() {
            s.clear();
        }
        self.regions.clear();
        self.delim_stack.clear();
        self.cache_index = 0;
        self.cache_pos = 0;
    }

    fn new_node(&mut self, tag: &str) -> usize {
        self.arena.new_node(tag)
    }

    /// `InlineProcessor.run(tree)` — exact BFS queue over the arena.
    /// Dedicated smarty pass (own pattern registry, tree priority 6).
    pub fn run_smarty(ctx: &'a mut Context, root: &mut Element) {
        let (arena, root_idx) = Arena::from_element(root);
        let mut inl = Inline::new(ctx);
        inl.arena = arena;
        inl.reset_run();
        let table = crate::ext::smarty::patterns_for(&*inl.ctx);
        inl.patterns = (0..table.len()).map(Pat::Smarty).collect();
        inl.smarty = table;
        inl.run_queue(root_idx, root);
    }

    /// Main inline pass.
    pub fn run(ctx: &'a mut Context, root: &mut Element) {
        let (arena, root_idx) = Arena::from_element(root);
        let mut inl = Inline::new(ctx);
        inl.arena = arena;
        inl.reset_run();
        inl.run_queue(root_idx, root);
    }

    /// BFS queue core shared by both passes.
    fn run_queue(&mut self, root_idx: usize, root: &mut Element) {
        let inl = self;
        // Seed parent_map for the whole tree (upstream builds it upfront).
        {
            let mut stack = vec![root_idx];
            while let Some(n) = stack.pop() {
                let kids = inl.arena.nodes[n].children.clone();
                for &k in &kids {
                    inl.parent_map.insert(k, n);
                    stack.push(k);
                }
            }
        }
        let mut queue: VecDeque<(usize, Vec<String>, bool)> = VecDeque::new();
        queue.push_back((root_idx, Vec::new(), true));
        while let Some((cur, parents, is_tree)) = queue.pop_front() {
            inl.ancestors = parents;
            if !is_tree {
                let tag = inl.arena.nodes[cur].tag.clone();
                inl.ancestors.push(tag.to_lowercase());
            }
            let kids: Vec<usize> = inl.arena.nodes[cur].children.clone();
            let mut insert_queue: Vec<crate::InsertQueueEntry> = Vec::new();
            for &child in &kids {
                // `if child.text and not isinstance(child.text, AtomicString)`.
                let text = inl.arena.nodes[child].text.clone();
                let do_text =
                    matches!(&text, Some(t) if !t.as_str().is_empty() && !t.is_atomic());
                if do_text {
                    let tag = inl.arena.nodes[child].tag.clone();
                    inl.ancestors.push(tag.to_lowercase());
                    inl.arena.nodes[child].text = None;
                    let text_s = text.unwrap().into_string();
                    let handled = inl.handle_inline(&text_s, 0);
                    let (lst, pending) =
                        inl.process_placeholders(Text::Normal(handled), child, true);
                    if let Some(p) = pending {
                        inl.arena.nodes[child].text = Some(p);
                    }
                    for (e, anc) in &lst {
                        inl.parent_map.insert(*e, child);
                        queue.push_back((*e, anc.clone(), false));
                    }
                    let lst_idx: Vec<(usize, Vec<String>)> = lst;
                    insert_queue.push((child, lst_idx));
                    inl.ancestors.pop();
                }
                // `if child.tail`.
                let tail = inl.arena.nodes[child].tail.clone();
                if let Some(t) = tail {
                    if !t.as_str().is_empty() && !t.is_atomic() {
                        let handled = inl.handle_inline(t.as_str(), 0);
                        let dumby = inl.new_node("d");
                        inl.arena.nodes[child].tail = None;
                        let (lst, pending) =
                            inl.process_placeholders(Text::Normal(handled), dumby, false);
                        if let Some(p) = pending {
                            inl.arena.nodes[child].tail = Some(p);
                        }
                        let pos = inl.arena.nodes[cur]
                            .children
                            .iter()
                            .position(|&c| c == child)
                            .unwrap()
                            + 1;
                        for (e, anc) in lst.iter().rev() {
                            inl.parent_map.insert(*e, cur);
                            inl.arena.nodes[cur].children.insert(pos, *e);
                            queue.push_back((*e, anc.clone(), false));
                        }
                    }
                }
                if !inl.arena.nodes[child].children.is_empty() {
                    inl.parent_map.insert(child, cur);
                    queue.push_back((child, inl.ancestors.clone(), false));
                }
            }
            for (element, lst) in insert_queue {
                for (i, (obj, _)) in lst.iter().enumerate() {
                    // `element.insert(i, newChild)`.
                    let children = &mut inl.arena.nodes[element].children;
                    let idx = i.min(children.len());
                    children.insert(idx, *obj);
                }
            }
        }
        let out = inl.arena.to_element(root_idx);
        *root = out;
    }
}
impl<'a> Inline<'a> {
    // ---- `__handleInline` ----

    fn handle_inline(&mut self, data: &str, pattern_index: usize) -> String {
        self.cur_mask = text_mask(data);
        // Whole-text early-out for the main pass (trigger bytes absent).
        if matches!(self.patterns.first(), Some(p) if !matches!(p, Pat::Smarty(_)))
            && self.cur_mask & self.main_union == 0
        {
            return data.to_string();
        }
        let mut data = data.to_string();
        let count = self.patterns.len();
        let mut pi = pattern_index;
        let mut pos = 0usize;
        while pi < count {
            let pat = self.patterns[pi];
            let (next, matched, resume) = self.apply_pattern(pat, &data, pi, pos);
            data = next;
            if matched {
                pos = resume;
            } else {
                pi += 1;
                pos = 0;
            }
        }
        data
    }

    // ---- `__applyPattern` (single accepted match per call) ----
    //
    // Returns `(data, matched, resume)`. `handle_inline` retries the same
    // pattern from `resume` until it reports no match.
    fn apply_pattern(
        &mut self,
        pat: Pat,
        data: &str,
        pattern_index: usize,
        start_index: usize,
    ) -> (String, bool, usize) {
        if self.excluded(pat) {
            return (data.to_string(), false, 0);
        }
        // Trigger prefilter (P6-adjacent): no trigger bytes, no match.
        // Safe: every match of these patterns contains its trigger
        // literal, and placeholders/STX sequences contain none.
        let need = match pat {
            Pat::Smarty(i) => self.smarty[i].1.mask,
            _ => pat_mask(pat),
        };
        if need & self.cur_mask == 0 {
            return (data.to_string(), false, 0);
        }
        let smarty_re;
        let re = match pat {
            Pat::Smarty(i) => {
                smarty_re = self.smarty[i].1.re.clone();
                &smarty_re
            }
            Pat::HtmlStrict => &*HTMLSTRICT,
            _ => pat_regex(pat),
        };
        let found: Vec<(usize, usize)> = find_iter_pos(re, data, start_index);
        for (ms, me) in found {
            match self.handle_match(pat, data, ms, me, pattern_index) {
                MatchOutcome::Reject => continue,
                MatchOutcome::Advance(end) => {
                    return (data.to_string(), true, end);
                }
                MatchOutcome::Accept { node, start, end } => {
                    if node_is_none(&node) {
                        return (data.to_string(), true, end);
                    }
                    // Recurse into the new node's text/tail with later
                    // patterns (`patternIndex + 1`), unless atomic.
                    let mut node = node;
                    self.process_new_node(&mut node, pattern_index + 1);
                    let ph = self.stash_node(node);
                    // Python slicing clamps (cached delimiter regions can
                    // overshoot after placeholder insertion); mirror that.
                    let new_data = format!(
                        "{}{}{}",
                        pyslice(data, 0, start as isize),
                        ph,
                        pyslice(data, end as isize, data.len() as isize)
                    );
                    return (new_data, true, start + ph.len());
                }
            }
        }
        (data.to_string(), false, 0)
    }

    fn excluded(&self, _pat: Pat) -> bool {
        // All core patterns have empty ANCESTOR_EXCLUDES; extension
        // patterns slot in here when `ext/` lands.
        false
    }

    // ---- stash ----

    fn stash_node(&mut self, node: NewNode) -> String {
        let id = self.next_id;
        self.next_id += 1;
        self.stashed.insert(id, node.into_stash(&mut self.arena));
        placeholder_inline(id)
    }

    // ---- `__processPlaceholders` ----
    //
    // Returns `(new children, pending parent text)`. `parent` is the
    // subnode whose text/tail is being expanded; pending text accumulates
    // there exactly like upstream `linkText`.
    fn process_placeholders(
        &mut self,
        data: Text,
        parent: usize,
        is_text: bool,
    ) -> (Vec<(usize, Vec<String>)>, Option<Text>) {
        let atomic_in = data.is_atomic();
        let data_s = data.into_string();
        let mut result: Vec<(usize, Vec<String>)> = Vec::new();
        // Pending parent text/tail (linkText target when `result` empty).
        let mut pending: Option<String> = None;
        let mut pending_atomic = false;
        // linkText helper: append `t` to last result tail, else pending.
        macro_rules! link_text {
            ($t:expr) => {{
                let t: String = $t;
                if t.is_empty() {
                } else if let Some((last, _)) = result.last_mut() {
                    let tail = &mut self.arena.nodes[*last].tail;
                    match tail {
                        Some(Text::Normal(s)) => s.push_str(&t),
                        Some(Text::Atomic(s)) => s.push_str(&t),
                        None => *tail = Some(Text::Normal(t)),
                    }
                } else if !is_text {
                    match &mut pending {
                        Some(p) => p.push_str(&t),
                        None => {
                            pending = Some(t);
                            pending_atomic = false;
                        }
                    }
                } else {
                    match &mut pending {
                        Some(p) => p.push_str(&t),
                        None => {
                            pending = Some(t);
                            pending_atomic = false;
                        }
                    }
                }
            }};
        }
        let mut start_index = 0usize;
        while !data_s.is_empty() {
            let idx = match data_s[start_index..].find(crate::consts::INLINE_PLACEHOLDER_PREFIX) {
                Some(r) => start_index + r,
                None => {
                    let text = data_s[start_index..].to_string();
                    // Atomic wrap only when nothing was emitted before
                    // (upstream `+=` degrades multi-segment remainders
                    // to plain `str`).
                    let first = pending.is_none() && result.is_empty();
                    link_text!(text);
                    if atomic_in && first {
                        pending_atomic = true;
                    }
                    break;
                }
            };
            let (id, ph_end) = match parse_ph(&data_s[idx..]) {
                Some(t) => t,
                None => {
                    // Wrong placeholder: emit through the prefix end.
                    let end = idx + crate::consts::INLINE_PLACEHOLDER_PREFIX.len();
                    let t = data_s[start_index..end].to_string();
                    link_text!(t);
                    start_index = end;
                    continue;
                }
            };
            let ph_end_abs = idx + ph_end;
            if let Some(node) = self.stashed.get(&id).cloned() {
                // Text before the placeholder (both branches).
                if idx > start_index {
                    let t = data_s[start_index..idx].to_string();
                    link_text!(t);
                }
                match node {
                    StashNode::Str(s) => {
                        link_text!(s);
                        start_index = ph_end_abs;
                        continue;
                    }
                    StashNode::El(n) => {
                        // Expand the stashed element's text/tail.
                        self.process_element_text_placeholder(n, parent);
                        start_index = ph_end_abs;
                        result.push((n, self.ancestors.clone()));
                    }
                }
            } else {
                // Wrong placeholder (unknown id).
                let end = idx + crate::consts::INLINE_PLACEHOLDER_PREFIX.len();
                let t = data_s[start_index..end].to_string();
                link_text!(t);
                start_index = end;
            }
        }
        let pending_text = pending.map(|p| {
            if pending_atomic {
                Text::Atomic(p)
            } else {
                Text::Normal(p)
            }
        });
        let _ = parent;
        (result, pending_text)
    }

    /// `__processElementText` for placeholder expansion (the subset used
    /// from `__processPlaceholders`): expand text/tail placeholders of a
    /// stashed element's children.
    fn process_element_text_placeholder(&mut self, node: usize, _parent: usize) {
        // `for child in [node] + list(node)`.
        let targets: Vec<usize> = std::iter::once(node)
            .chain(self.arena.nodes[node].children.clone())
            .collect();
        for child in targets {
            let tail = self.arena.nodes[child].tail.clone();
            if let Some(t) = tail {
                if !t.as_str().trim().is_empty() {
                    self.arena.nodes[child].tail = None;
                    let (lst, pending) =
                        self.process_placeholders(t, child, false);
                    if let Some(p) = pending {
                        self.arena.nodes[child].tail = Some(p);
                    }
                    // Tails expand into `node` after `child`
                    // (`node.insert(index+1, ...)`; pos=0 when
                    // node is the subnode itself).
                    let pos = self.arena.nodes[node]
                        .children
                        .iter()
                        .position(|&c| c == child)
                        .map(|i| i + 1)
                        .unwrap_or(0);
                    for (e, _) in lst.iter().rev() {
                        self.arena.nodes[node].children.insert(pos, *e);
                    }
                }
            }
            let text = self.arena.nodes[child].text.clone();
            if let Some(t) = text {
                if !t.as_str().trim().is_empty() {
                    self.arena.nodes[child].text = None;
                    let (lst, pending) =
                        self.process_placeholders(t, child, true);
                    if let Some(p) = pending {
                        self.arena.nodes[child].text = Some(p);
                    }
                    for (e, anc) in lst.iter().rev() {
                        self.arena.nodes[child].children.insert(0, *e);
                        let _ = anc;
                    }
                }
            }
        }
    }
}

/// Parse `klzzwxh:NNNN\x03` at the start of `s`: `(id, len)`.
fn parse_ph(s: &str) -> Option<(usize, usize)> {
    let rest = s.strip_prefix(crate::consts::INLINE_PLACEHOLDER_PREFIX)?;
    let mut e = 0;
    while e < rest.len() && rest.as_bytes()[e].is_ascii_digit() {
        e += 1;
    }
    if e < rest.len() && rest.as_bytes()[e] == 3 {
        let id: usize = rest[..e].parse().ok()?;
        Some((id, crate::consts::INLINE_PLACEHOLDER_PREFIX.len() + e + 1))
    } else {
        None
    }
}

fn next_char(s: &str, pos: usize) -> usize {
    let mut e = (pos + 1).min(s.len());
    while e < s.len() && !s.is_char_boundary(e) {
        e += 1;
    }
    e
}

/// New node from `handleMatch`: owned element or raw string.
#[derive(Debug, Clone)]
enum NewNode {
    El(Element),
    Str(String),
    None,
}

fn node_is_none(n: &NewNode) -> bool {
    matches!(n, NewNode::None)
}

impl NewNode {
    fn into_stash(self, arena: &mut Arena) -> StashNode {
        match self {
            NewNode::El(el) => {
                let idx = arena.push_element(&el);
                StashNode::El(idx)
            }
            NewNode::Str(s) => StashNode::Str(s),
            NewNode::None => StashNode::Str(String::new()),
        }
    }
}

/// Outcome of one `handleMatch` call.
enum MatchOutcome {
    /// `(None, None, None)`: try the next match.
    Reject,
    /// `(None, start, end)`: consume through `end`, retry same pattern.
    Advance(usize),
    /// `(node|None, start, end)`: accept.
    Accept { node: NewNode, start: usize, end: usize },
}
impl<'a> Inline<'a> {
    // ---- `process_new_node` (applyPattern recursion into new nodes) ----

    fn process_new_node(&mut self, node: &mut NewNode, pattern_index: usize) {
        if pattern_index >= self.patterns.len() {
            return;
        }
        let NewNode::El(el) = node else {
            return;
        };
        // `if not isinstance(node.text, AtomicString)` gates the whole loop.
        if matches!(&el.text, Some(t) if t.is_atomic()) {
            return;
        }
        // Recursive `handle_inline` calls reset `cur_mask` for their
        // substring; save/restore ours (the outer scan continues).
        let saved_mask = self.cur_mask;
        // (Borrow-safe: process texts first, then tails.)
        let nch = el.children.len();
        // node itself (index None) + children 0..nch.
        for ci in 0..nch + 1 {
            let tag = if ci == 0 {
                el.tag.clone()
            } else {
                el.children[ci - 1].tag.clone()
            };
            self.ancestors.push(tag.to_lowercase());
            let t = if ci == 0 {
                el.text.clone()
            } else {
                el.children[ci - 1].text.clone()
            };
            if let Some(t) = t {
                if !t.as_str().is_empty() {
                    let h = self.handle_inline(t.as_str(), pattern_index);
                    let h = if t.is_atomic() {
                        Text::Atomic(h)
                    } else {
                        Text::Normal(h)
                    };
                    if ci == 0 {
                        el.text = Some(h);
                    } else {
                        el.children[ci - 1].text = Some(h);
                    }
                }
            }
            self.ancestors.pop();
            let t = if ci == 0 {
                el.tail.clone()
            } else {
                el.children[ci - 1].tail.clone()
            };
            if let Some(t) = t {
                if !t.as_str().is_empty() {
                    let h = self.handle_inline(t.as_str(), pattern_index);
                    if ci == 0 {
                        el.tail = Some(Text::Normal(h));
                    } else {
                        el.children[ci - 1].tail = Some(Text::Normal(h));
                    }
                }
            }
        }
        self.cur_mask = saved_mask;
    }

    // ---- `Pattern.unescape` (text-only) ----

    fn unescape_text_pat(&self, text: &str) -> String {
        let re = &crate::consts::RE_INLINE_PH;
        re.replace_all(text, |caps: &Captures| {
            let id: usize = caps
                .get(1)
                .map(|m| m.as_str().parse().unwrap_or(usize::MAX))
                .unwrap_or(usize::MAX);
            match self.stashed.get(&id) {
                // Upstream splices stashed strings but leaves element
                // placeholders intact (`return m.group(0)`).
                Some(StashNode::Str(s)) => s.clone(),
                Some(StashNode::El(_)) | None => caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
            }
        })
        .into_owned()
    }

    // ---- `HtmlInlineProcessor.unescape` (serializing) ----

    fn unescape_html(&self, text: &str) -> String {
        let re = &crate::consts::RE_INLINE_PH;
        re.replace_all(text, |caps: &Captures| {
            let id: usize = caps
                .get(1)
                .map(|m| m.as_str().parse().unwrap_or(usize::MAX))
                .unwrap_or(usize::MAX);
            match self.stashed.get(&id) {
                Some(StashNode::El(n)) => {
                    let el = self.arena.to_element(*n);
                    let s = crate::ser::serialize(&el, self.ctx.output_format);
                    self.unescape_html(&s)
                }
                Some(StashNode::Str(s)) => format!("\\{s}"),
                None => caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
            }
        })
        .into_owned()
    }

    fn backslash_unescape(&self, text: &str) -> String {
        crate::consts::RE_UNESCAPE
            .replace_all(text, |caps: &Captures| {
                let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
                char::from_u32(n).unwrap_or('\u{FFFD}').to_string()
            })
            .into_owned()
    }

    // ---- `handleMatch` dispatch ----

    fn handle_match(
        &mut self,
        pat: Pat,
        data: &str,
        ms: usize,
        me: usize,
        _pattern_index: usize,
    ) -> MatchOutcome {
        match pat {
            Pat::Backtick => self.match_backtick(data, ms, me),
            Pat::Escape => self.match_escape(data, ms, me),
            Pat::Reference => self.match_reference(data, ms, me, false),
            Pat::Link => self.match_link(data, ms, me),
            Pat::ImageLink => self.match_image(data, ms, me, false),
            Pat::ImageReference => self.match_image(data, ms, me, true),
            Pat::ShortReference => self.match_short_reference(data, ms, me, false),
            Pat::ShortImageReference => self.match_short_reference(data, ms, me, true),
            Pat::Autolink => self.match_autolink(data, ms, me),
            Pat::Automail => self.match_automail(data, ms, me),
            Pat::Linebreak => MatchOutcome::Accept {
                node: NewNode::El(Element::new("br")),
                start: ms,
                end: me,
            },
            Pat::Html | Pat::Entity => self.match_html(pat, data, ms, me),
            Pat::EmStrong => self.match_delimiter(data, ms, me),
            Pat::Footnote => self.match_footnote(data, ms, me),
            Pat::Smarty(i) => self.match_smarty(i, data, ms, me),
            Pat::HtmlStrict => self.match_html(Pat::HtmlStrict, data, ms, me),
            Pat::WikiLink => self.match_wikilink(data, ms, me),
            Pat::Nl => MatchOutcome::Accept {
                node: NewNode::El(Element::new("br")),
                start: ms,
                end: me,
            },
        }
    }

    fn caps<'d>(&self, pat: Pat, data: &'d str, ms: usize) -> Captures<'d> {
        pat_regex(pat).captures_from_pos(data, ms).unwrap().unwrap()
    }

    // ---- BacktickInlineProcessor ----

    fn match_backtick(&mut self, data: &str, ms: usize, me: usize) -> MatchOutcome {
        let caps = self.caps(Pat::Backtick, data, ms);
        if let Some(g1) = caps.get(1) {
            let escaped = format!("{}92{}", '\u{2}', '\u{3}');
            let s = g1.as_str().replace("\\\\", &escaped);
            return MatchOutcome::Accept {
                node: NewNode::Str(s),
                start: ms,
                end: me,
            };
        }
        match find_code_spans(ms, data) {
            Some((start, end)) => {
                let mut el = Element::new("code");
                el.text = Some(Text::Atomic(crate::consts::code_escape(
                    data[start..end].trim(),
                )));
                // `result[1] + (start - begin)`.
                let endpos = end + (start - ms);
                MatchOutcome::Accept {
                    node: NewNode::El(el),
                    start: ms,
                    end: endpos,
                }
            }
            None => MatchOutcome::Reject,
        }
    }

    // ---- EscapeInlineProcessor ----

    fn match_escape(&mut self, data: &str, ms: usize, me: usize) -> MatchOutcome {
        let caps = self.caps(Pat::Escape, data, ms);
        let ch = caps.get(1).unwrap().as_str();
        let ch = ch.chars().next().unwrap();
        if self.ctx.escaped_chars().contains(&ch) {
            MatchOutcome::Accept {
                node: NewNode::Str(format!("{}{}{}", '\u{2}', ch as u32, '\u{3}')),
                start: ms,
                end: me,
            }
        } else {
            MatchOutcome::Advance(me)
        }
    }
}
/// `BacktickInlineProcessor.find_code_spans` (byte indices; backticks are
/// ASCII so byte/char positions coincide on the scanned runs).
fn find_code_spans(begin: usize, text: &str) -> Option<(usize, usize)> {
    let b = text.as_bytes();
    let last = b.len();
    let mut s = begin;
    let mut max_ticks = 0usize;
    while s < last && b[s] == b'`' {
        max_ticks += 1;
        s += 1;
    }
    if max_ticks == 0 {
        return None;
    }
    let mut longest = 0usize;
    let mut end = 0usize;
    let mut i = s;
    while i < last {
        let mut span = 0usize;
        while i < last && b[i] == b'`' {
            span += 1;
            i += 1;
        }
        if span == 0 {
            i += 1;
            continue;
        }
        if max_ticks == span {
            return Some((s, i - span));
        }
        if span > longest {
            longest = span;
            end = i;
        }
    }
    if longest > 0 {
        return Some((s - (max_ticks - longest), end - longest));
    }
    None
}
impl<'a> Inline<'a> {
    // ---- LinkInlineProcessor ----

    fn match_link(&mut self, data: &str, ms: usize, _me: usize) -> MatchOutcome {
        let (text, index, handled) = get_text(data, self.caps_end(Pat::Link, data, ms));
        if !handled {
            return MatchOutcome::Reject;
        }
        let (href, title, index, handled) = self.get_link(data, index);
        if !handled {
            return MatchOutcome::Reject;
        }
        let mut el = Element::new("a");
        el.text = Some(Text::Normal(text));
        el.set_attr("href", &href);
        if let Some(t) = title {
            el.set_attr("title", &t);
        }
        MatchOutcome::Accept {
            node: NewNode::El(el),
            start: ms,
            end: index,
        }
    }

    fn caps_end(&self, pat: Pat, data: &str, ms: usize) -> usize {
        let caps = pat_regex(pat).captures_from_pos(data, ms).unwrap().unwrap();
        caps.get(0).unwrap().end()
    }

    /// `getLink(data, index)`.
    fn get_link(&self, data: &str, index: usize) -> (String, Option<String>, usize, bool) {
        let re = compile(P::RE_IL_LINKINLINEPROCESSOR_RE_LINK);
        let m = match re.captures_from_pos(data, index) {
            Ok(Some(m)) if m.get(0).unwrap().start() == index => m,
            _ => return (String::new(), None, index, false),
        };
        // Shared tail (placeholder-unescape + title cleanup) applies to
        // both the `<url>` fast path and the nesting machine, like
        // upstream's fall-through to `href = self.unescape(href)`.
        let mut href = String::new();
        let mut title: Option<String> = None;
        let mut index: usize;
        let handled: bool;
        if let Some(g1) = m.get(1) {
            let raw = g1.as_str();
            href = raw[1..raw.len() - 1].trim().to_string();
            title = m.get(2).map(|g| {
                let t = g.as_str();
                t[1..t.len() - 1].to_string()
            });
            index = m.get(0).unwrap().end();
            handled = true;
        } else {
        // Bracket-nesting state machine (mechanical port; byte indices,
        // ASCII delimiters only).
        let b = data.as_bytes();
        let n = b.len();
        let mut bracket_count: i64 = 1;
        let mut backtrack_count: i64 = 1;
        let start_index = m.get(0).unwrap().end();
        index = start_index;
        let mut last_bracket: i64 = -1;
        let mut quote: u8 = 0;
        let mut start_quote: i64 = -1;
        let mut exit_quote: i64 = -1;
        let mut ignore_matches = false;
        let mut alt_quote: u8 = 0;
        let mut start_alt_quote: i64 = -1;
        let mut exit_alt_quote: i64 = -1;
        let mut last: u8 = 0;
        let mut pos = index;
        while pos < n {
            let c = b[pos];
            if c == b'(' {
                if !ignore_matches {
                    bracket_count += 1;
                } else if backtrack_count > 0 {
                    backtrack_count -= 1;
                }
            } else if c == b')' {
                if (exit_quote != -1 && quote == last)
                    || (exit_alt_quote != -1 && alt_quote == last)
                {
                    bracket_count = 0;
                } else if !ignore_matches {
                    bracket_count -= 1;
                } else if backtrack_count > 0 {
                    backtrack_count -= 1;
                    if backtrack_count == 0 {
                        last_bracket = index as i64 + 1;
                    }
                }
            } else if c == b'\'' || c == b'"' {
                if quote == 0 {
                    ignore_matches = true;
                    backtrack_count = bracket_count;
                    bracket_count = 1;
                    start_quote = index as i64 + 1;
                    quote = c;
                } else if c != quote && alt_quote == 0 {
                    start_alt_quote = index as i64 + 1;
                    alt_quote = c;
                } else if c == quote {
                    exit_quote = index as i64 + 1;
                } else if alt_quote != 0 && c == alt_quote {
                    exit_alt_quote = index as i64 + 1;
                }
            }
            index += 1;
            if bracket_count == 0 {
                if exit_quote >= 0 && quote == last {
                    href = data[start_index..(start_quote - 1) as usize].to_string();
                    title = Some(data[(start_quote as usize)..(exit_quote - 1) as usize].to_string());
                } else if exit_alt_quote >= 0 && alt_quote == last {
                    href = data[start_index..(start_alt_quote - 1) as usize].to_string();
                    title = Some(
                        data[(start_alt_quote as usize)..(exit_alt_quote - 1) as usize].to_string(),
                    );
                } else {
                    href = data[start_index..index - 1].to_string();
                }
                break;
            }
            if c != b' ' {
                last = c;
            }
            pos += 1;
        }
        if bracket_count != 0 && backtrack_count == 0 && last_bracket > 0 {
            href = data[start_index..(last_bracket - 1) as usize].to_string();
            index = last_bracket as usize;
            bracket_count = 0;
        }
        handled = bracket_count == 0;
        } // end else (nesting machine); shared tail below mirrors
          // upstream's fall-through to `href = self.unescape(href)`.
        if title.is_some() {
            let t = title.take().unwrap();
            let t = self.unescape_text_pat(t.trim());
            title = Some(clean_title(&dequote(&t)));
        }
        href = self.unescape_text_pat(&href).trim().to_string();
        (href, title, index, handled)
    }

    // ---- ImageInlineProcessor / ReferenceInlineProcessor ----

    fn match_image(&mut self, data: &str, ms: usize, _me: usize, is_ref: bool) -> MatchOutcome {
        let end = self.caps_end(if is_ref { Pat::ImageReference } else { Pat::ImageLink }, data, ms);
        let (text, index, handled) = get_text(data, end);
        if !handled {
            return MatchOutcome::Reject;
        }
        if !is_ref {
            let (src, title, index, handled) = self.get_link(data, index);
            if !handled {
                return MatchOutcome::Reject;
            }
            let mut el = Element::new("img");
            el.set_attr("src", &src);
            if let Some(t) = title {
                el.set_attr("title", &t);
            }
            el.set_attr("alt", &self.unescape_text_pat(&text));
            return MatchOutcome::Accept {
                node: NewNode::El(el),
                start: ms,
                end: index,
            };
        }
        let (id, end, handled) = self.eval_id(data, index, &text, true);
        if !handled {
            return MatchOutcome::Reject;
        }
        self.make_image_tag(&id, &text, ms, end)
    }

    fn match_reference(&mut self, data: &str, ms: usize, _me: usize, _is_image: bool) -> MatchOutcome {
        let end = self.caps_end(Pat::Reference, data, ms);
        let (text, index, handled) = get_text(data, end);
        if !handled {
            return MatchOutcome::Reject;
        }
        let (id, end, handled) = self.eval_id(data, index, &text, false);
        if !handled {
            return MatchOutcome::Reject;
        }
        let id = clean_newlines(&id);
        if let Some((href, title)) = self.ctx.references.get(&id).cloned() {
            let mut el = Element::new("a");
            el.set_attr("href", &href);
            if let Some(t) = title {
                el.set_attr("title", &t);
            }
            el.text = Some(Text::Normal(text));
            MatchOutcome::Accept {
                node: NewNode::El(el),
                start: ms,
                end,
            }
        } else {
            MatchOutcome::Accept {
                node: NewNode::None,
                start: ms,
                end,
            }
        }
    }

    fn match_short_reference(
        &mut self,
        data: &str,
        ms: usize,
        _me: usize,
        is_image: bool,
    ) -> MatchOutcome {
        let pat = if is_image { Pat::ShortImageReference } else { Pat::ShortReference };
        let end = self.caps_end(pat, data, ms);
        let (text, index, handled) = get_text(data, end);
        if !handled {
            return MatchOutcome::Reject;
        }
        let id = text.to_lowercase();
        let id = clean_newlines(&id);
        if is_image {
            return self.make_image_tag(&id, &text, ms, index);
        }
        if let Some((href, title)) = self.ctx.references.get(&id).cloned() {
            let mut el = Element::new("a");
            el.set_attr("href", &href);
            if let Some(t) = title {
                el.set_attr("title", &t);
            }
            el.text = Some(Text::Normal(text));
            MatchOutcome::Accept {
                node: NewNode::El(el),
                start: ms,
                end: index,
            }
        } else {
            MatchOutcome::Accept {
                node: NewNode::None,
                start: ms,
                end: index,
            }
        }
    }

    fn eval_id(&self, data: &str, index: usize, text: &str, _is_image: bool) -> (String, usize, bool) {
        let re = compile(P::RE_IL_REFERENCEINLINEPROCESSOR_RE_LINK);
        let m = match re.captures_from_pos(data, index) {
            Ok(Some(m)) if m.get(0).unwrap().start() == index => m,
            _ => return (String::new(), index, false),
        };
        let mut id = m.get(1).unwrap().as_str().to_lowercase();
        let end = m.get(0).unwrap().end();
        if id.is_empty() {
            id = text.to_lowercase();
        }
        (id, end, true)
    }

    fn make_image_tag(
        &mut self,
        id: &str,
        text: &str,
        ms: usize,
        end: usize,
    ) -> MatchOutcome {
        let id = clean_newlines(id);
        if let Some((href, title)) = self.ctx.references.get(&id).cloned() {
            let mut el = Element::new("img");
            el.set_attr("src", &href);
            if let Some(t) = title {
                el.set_attr("title", &t);
            }
            el.set_attr("alt", &self.unescape_text_pat(text));
            MatchOutcome::Accept {
                node: NewNode::El(el),
                start: ms,
                end,
            }
        } else {
            MatchOutcome::Accept {
                node: NewNode::None,
                start: ms,
                end,
            }
        }
    }
}

/// `getText`: bracket-balanced scan from `index`.
fn get_text(data: &str, index: usize) -> (String, usize, bool) {
    let b = data.as_bytes();
    let mut bracket_count: i64 = 1;
    let mut text = Vec::new();
    let mut index = index;
    let mut pos = index;
    while pos < b.len() {
        let c = b[pos];
        if c == b']' {
            bracket_count -= 1;
        } else if c == b'[' {
            bracket_count += 1;
        }
        index += 1;
        if bracket_count == 0 {
            break;
        }
        text.push(c);
        pos += 1;
    }
    (String::from_utf8(text).unwrap(), index, bracket_count == 0)
}

fn dequote(s: &str) -> String {
    if (s.starts_with('"') && s.ends_with('"') && s.len() >= 2)
        || (s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2)
    {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn clean_title(s: &str) -> String {
    s.chars().map(|c| if c.is_whitespace() { ' ' } else { c }).collect()
}

fn clean_newlines(s: &str) -> String {
    // `NEWLINE_CLEANUP_RE = \s+` (multiline) -> ' '.
    let mut out = String::new();
    let mut was_ws = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if !was_ws {
                out.push(' ');
            }
            was_ws = true;
        } else {
            out.push(c);
            was_ws = false;
        }
    }
    out
}
impl<'a> Inline<'a> {
    // ---- AutolinkInlineProcessor ----

    fn match_autolink(&mut self, data: &str, ms: usize, me: usize) -> MatchOutcome {
        let caps = self.caps(Pat::Autolink, data, ms);
        let g1 = caps.get(1).unwrap().as_str().to_string();
        let mut el = Element::new("a");
        el.set_attr("href", &self.unescape_text_pat(&g1));
        el.text = Some(Text::Atomic(g1));
        MatchOutcome::Accept {
            node: NewNode::El(el),
            start: ms,
            end: me,
        }
    }

    // ---- AutomailInlineProcessor ----

    fn match_automail(&mut self, data: &str, ms: usize, me: usize) -> MatchOutcome {
        let caps = self.caps(Pat::Automail, data, ms);
        let mut email = self.unescape_text_pat(caps.get(1).unwrap().as_str());
        if let Some(stripped) = email.strip_prefix("mailto:") {
            email = stripped.to_string();
        }
        let letters: String = email
            .chars()
            .map(|c| {
                let code = c as u32;
                match crate::entities_gen::CODEPOINT2NAME.binary_search_by(|e| e.0.cmp(&code)) {
                    Ok(i) => format!("{}{};", AMP_SUBSTITUTE, crate::entities_gen::CODEPOINT2NAME[i].1),
                    Err(_) => format!("{AMP_SUBSTITUTE}#{code};"),
                }
            })
            .collect();
        let mut el = Element::new("a");
        let mailto = format!("mailto:{email}");
        let href: String = mailto
            .chars()
            .map(|c| format!("{AMP_SUBSTITUTE}#{};", c as u32))
            .collect();
        el.set_attr("href", &href);
        el.text = Some(Text::Atomic(letters));
        MatchOutcome::Accept {
            node: NewNode::El(el),
            start: ms,
            end: me,
        }
    }

    // ---- HtmlInlineProcessor (html + entity) ----

    fn match_html(&mut self, pat: Pat, data: &str, ms: usize, me: usize) -> MatchOutcome {
        let re = match pat {
            Pat::HtmlStrict => &*HTMLSTRICT,
            _ => pat_regex(pat),
        };
        let caps = re.captures_from_pos(data, ms).unwrap().unwrap();
        let g1 = caps.get(1).unwrap().as_str().to_string();
        let rawhtml = self.backslash_unescape(&self.unescape_html(&g1));
        let key = self.ctx.stash.len();
        self.ctx
            .stash
            .push(crate::context::StashEntry::Html(rawhtml));
        MatchOutcome::Accept {
            node: NewNode::Str(crate::consts::placeholder_html(key)),
            start: ms,
            end: me,
        }
    }
}
/// `Delimiter._build_patterns` replicated exactly (machine-checked against
/// `delim_*` dumped consts for all four (token, smart) combos).
fn delim_boundary(token: &str, smart: bool) -> (String, String) {
    // `re.escape(token)`: alphanumerics and `_` pass through unescaped.
    let etoken: String = token
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c.to_string()
            } else {
                format!("\\{c}")
            }
        })
        .collect();
    let (xstart, xend) = if token != "_" {
        (
            format!(r"(?:(?<=_)|(?<![\w{etoken}]))"),
            format!(r"(?:(?=_)|(?![\w{etoken}]))"),
        )
    } else {
        (
            format!(r"(?<![\w{etoken}])"),
            format!(r"(?![\w{etoken}])"),
        )
    };
    let boundary = if smart {
        format!(
            r"(?x)(?P<ambiguous>(?<!^)(?<![\s{etoken}]){xstart}{etoken}{{1,}}{xend}(?![\s{etoken}])(?!$))|(?P<end>(?<!^)(?<![\s{etoken}]){etoken}{{1,}}{xend})|(?P<start>{xstart}{etoken}{{1,}}(?![\s{etoken}])(?!$))"
        )
    } else {
        format!(
            r"(?x)(?P<ambiguous>(?<!^)(?<![\s{etoken}]){etoken}{{1,}}(?![\s{etoken}])(?!$))|(?P<end>(?<!^)(?<![\s{etoken}]){etoken}{{1,}})|(?P<start>{etoken}{{1,}}(?![\s{etoken}])(?!$))"
        )
    };
    (boundary, format!("{etoken}+"))
}

#[cfg(test)]
mod delim_tests {
    use super::delim_boundary;
    use crate::patterns_gen as P;

    #[test]
    fn boundary_builder_matches_dump() {
        // The dump preserves upstream `(?x)` verbose formatting
        // (newlines/indentation); the builder emits the compact form.
        // Normalize before comparing (these patterns contain no
        // significant literal whitespace).
        fn norm(s: &str) -> String {
            let mut t: String = s.chars().filter(|c| !c.is_whitespace()).collect();
            if t.starts_with("(?x)(?x)") {
                t = "(?x)".to_string() + &t[8..];
            }
            t
        }
        for (tok, smart, name) in [
            ("*", false, "RE_DELIM_STAR_DUMB_BOUNDARY"),
            ("*", true, "RE_DELIM_STAR_SMART_BOUNDARY"),
            ("_", false, "RE_DELIM_US_DUMB_BOUNDARY"),
            ("_", true, "RE_DELIM_US_SMART_BOUNDARY"),
        ] {
            let (b, bad) = delim_boundary(tok, smart);
            let dump_b: &str = match name {
                "RE_DELIM_STAR_DUMB_BOUNDARY" => P::RE_DELIM_STAR_DUMB_BOUNDARY,
                "RE_DELIM_STAR_SMART_BOUNDARY" => P::RE_DELIM_STAR_SMART_BOUNDARY,
                "RE_DELIM_US_DUMB_BOUNDARY" => P::RE_DELIM_US_DUMB_BOUNDARY,
                _ => P::RE_DELIM_US_SMART_BOUNDARY,
            };
            let dump_bad: &str = match name {
                "RE_DELIM_STAR_DUMB_BOUNDARY" => P::RE_DELIM_STAR_DUMB_BAD,
                "RE_DELIM_STAR_SMART_BOUNDARY" => P::RE_DELIM_STAR_SMART_BAD,
                "RE_DELIM_US_DUMB_BOUNDARY" => P::RE_DELIM_US_DUMB_BAD,
                _ => P::RE_DELIM_US_SMART_BAD,
            };
            assert_eq!(norm(&b), norm(dump_b), "{name} boundary");
            assert_eq!(norm(&bad), norm(dump_bad), "{name} bad");
        }
    }
}
/// `finditer(data, start)` with Python-`re` empty-match semantics,
/// collected upfront (the data is immutable during the scan loop).
fn find_iter_pos(re: &F, data: &str, start: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut pos = start.min(data.len());
    while pos <= data.len() {
        let m = match re.find_from_pos(data, pos) {
            Ok(m) => m,
            Err(_) => break,
        };
        match m {
            Some(m) => {
                out.push((m.start(), m.end()));
                if m.end() == m.start() {
                    pos = next_char(data, m.end());
                } else {
                    pos = m.end();
                }
            }
            None => break,
        }
    }
    out
}
impl<'a> Inline<'a> {
    // ---- DelimiterProcessor ----

    fn delim_boundary_re(&mut self, token: &str) -> F {
        let smart = self
            .ctx
            .delimiter_tokens
            .iter()
            .find(|(t, _)| t == token)
            .map(|(_, s)| *s)
            .unwrap_or(false);
        let (b, _) = delim_boundary(token, smart);
        compile(&b)
    }

    fn delim_stack_of(&mut self, token: &str) -> &mut Vec<DelimEntry> {
        if !self.delim_stacks.iter().any(|(t, _)| t == token) {
            self.delim_stacks.push((token.to_string(), Vec::new()));
        }
        &mut self
            .delim_stacks
            .iter_mut()
            .find(|(t, _)| t == token)
            .unwrap()
            .1
    }

    /// `get_match(data, start)`: first delimiter-boundary match at `start`.
    fn delim_get_match(&mut self, data: &str, start: usize) -> Option<(usize, usize, String)> {
        let toks: Vec<String> = self.delim_stacks.iter().map(|(t, _)| t.clone()).collect();
        for tok in &toks {
            let re = self.delim_boundary_re(tok);
            if let Ok(Some(m)) = re.captures_from_pos(data, start) {
                if m.get(0).unwrap().start() == start {
                    let g = last_group_name(&m);
                    return Some((m.get(0).unwrap().start(), m.get(0).unwrap().end(), g));
                }
            }
        }
        None
    }

    /// `search(data, start)`: next viable opener/closer at/after `start`.
    fn delim_search(&mut self, data: &str, start: usize) -> Option<(usize, usize, String, String)> {
        let b = data.as_bytes();
        let mut i = start.min(b.len());
        while i < b.len() {
            let c = b[i] as char;
            let tok = if c == '*' {
                "*"
            } else if c == '_' {
                "_"
            } else {
                i += 1;
                continue;
            };
            if !self.delim_stacks.iter().any(|(t, _)| t == tok) {
                i += 1;
                continue;
            }
            let re = self.delim_boundary_re(tok);
            if let Ok(Some(m)) = re.captures_from_pos(data, i) {
                if m.get(0).unwrap().start() == i {
                    let g = last_group_name(&m);
                    let empty_stack = self
                        .delim_stacks
                        .iter()
                        .find(|(t, _)| t == tok)
                        .map(|(_, s)| s.is_empty())
                        .unwrap_or(true);
                    if !empty_stack || !g.starts_with('e') {
                        return Some((i, m.get(0).unwrap().end(), g, tok.to_string()));
                    }
                }
            }
            i += 1;
        }
        None
    }

    fn add_region(&mut self, tok: &str, a: usize, b: usize, c: usize, d: usize, size: usize) {
        let tags = self
            .ctx
            .delimiter_tokens
            .iter()
            .find(|(t, _)| t == tok)
            .map(|_| ("strong".to_string(), "em".to_string()))
            .unwrap_or(("strong".to_string(), "em".to_string()));
        self.regions.push(Region { a, b, c, d, tags, size });
        let toks: Vec<String> = self.delim_stacks.iter().map(|(t, _)| t.clone()).collect();
        for t in &toks {
            while let Some(e) = self.delim_stacks.iter().find(|(x, _)| x == t).unwrap().1.last() {
                let p = e.start;
                if p.max(a) <= p.min(d.saturating_sub(1)) {
                    self.delim_stacks.iter_mut().find(|(x, _)| x == t).unwrap().1.pop();
                } else {
                    break;
                }
            }
        }
    }

    fn match_delimiter(&mut self, data: &str, ms: usize, me: usize) -> MatchOutcome {
        // Cached regions pending?
        if !self.regions.is_empty() {
            let (el, start, end) = self.get_cached_result(data, ms);
            return MatchOutcome::Accept {
                node: NewNode::El(el),
                start,
                end,
            };
        }
        let m2 = self.delim_get_match(data, ms);
        let (m2s, m2e, m2g) = match m2 {
            Some(t) => t,
            None => {
                // Advance past the full bad run.
                let tok = trigger_token(data, ms);
                let (_, bad) = delim_boundary(&tok, false);
                let bad_re = compile(&bad);
                let be = bad_re
                    .find_from_pos(data, ms)
                    .ok()
                    .flatten()
                    .map(|m| m.end())
                    .unwrap_or(ms + 1);
                return MatchOutcome::Advance(be);
            }
        };
        if m2g.starts_with('e') {
            return MatchOutcome::Advance(m2e);
        }
        let token = trigger_token(data, ms);
        let (mut start, mut end) = (m2s, m2e);
        let length = end - start;
        // `double` is always false for core configs.
        let is_ambiguous = !m2g.starts_with('s');
        self.delim_stack_of(&token).push(DelimEntry {
            start,
            end: start + length,
            is_ambiguous,
            length,
        });
        let _ = end;
        // Pair tokens until stacks drain or no more matches.
        loop {
            if !self.delim_stacks.iter().any(|(_, s)| !s.is_empty()) {
                break;
            }
            let found = self.delim_search(data, end);
            let (fms, fme, fg, ftok) = match found {
                Some(t) => t,
                None => break,
            };
            let _ = fms;
            let token = ftok.clone();
            start = fms;
            end = fme;
            let mut current = fme - fms;
            let mut is_start = !fg.starts_with('e');
            let mut is_end = !is_start || !fg.starts_with('s');
            let is_ambiguous = is_start && is_end;
            let mut last = self
                .delim_stack_of(&token)
                .last()
                .map(|e| e.length)
                .unwrap_or(0);
            // Branch 1: consume previous starts.
            if !self.delim_stack_of(&token).is_empty()
                && is_end
                && (( !is_ambiguous && current > last) || current == last || current >= 3)
            {
                is_start = false;
                let original = current;
                let mut broke = false;
                while !self.delim_stack_of(&token).is_empty() && current > 0 && last <= current {
                    let delimiter = self.delim_stack_of(&token).pop().unwrap();
                    let max_size = 2usize;
                    let size = delimiter.length.min(if delimiter.length == 3 { 1 } else { max_size });
                    let a = delimiter.end - size;
                    self.add_region(&token, a, delimiter.end, start, start + size, size);
                    start += size;
                    current -= size;
                    // Push back the unconsumed remainder (upstream's
                    // `new`; `double` is always false in core configs).
                    if size < delimiter.length {
                        let new = delimiter.length - size;
                        self.delim_stack_of(&token).push(DelimEntry {
                            start: delimiter.start,
                            end: delimiter.end - size,
                            is_ambiguous: delimiter.is_ambiguous,
                            length: new,
                        });
                    }
                    if self.delim_stack_of(&token).is_empty() {
                        if self.delim_stacks.iter().any(|(t, s)| t != &token && !s.is_empty()) {
                            // token's stack (regions and sibling stacks kept).
                            self.delim_stack_of(&token).clear();
                            // `continue` the outer pairing loop.
                            broke = true;
                            // emulate `continue` via flag + break to loop head
                            break;
                        }
                        break;
                    }
                    last = self.delim_stack_of(&token).last().map(|e| e.length).unwrap_or(0);
                }
                if broke {
                    continue;
                }
                if original >= 3 && current > 0 && is_ambiguous {
                    self.delim_stack_of(&token).push(DelimEntry {
                        start: fms + (original - current),
                        end,
                        is_ambiguous: false,
                        length: current,
                    });
                    is_end = false;
                } else {
                    let nonempty = !self.delim_stack_of(&token).is_empty();
                    is_end = current > 0 && nonempty && last > current;
                }
            } // end branch 1 (three independent ifs upstream)
            // Branch 2 (***em* / ***strong** / **em*).
            match handle_branch2(
                self, data, &token, &mut start, &mut end, &mut current, &mut last,
                &mut is_end, &mut is_start, is_ambiguous,
            ) {
                BranchFlow::Break => break,
                BranchFlow::Continue => continue,
                BranchFlow::Done => {}
            }
            // Find opening tokens.
            if is_start {
                self.delim_stack_of(&token).push(DelimEntry {
                    start,
                    end,
                    is_ambiguous,
                    length: current,
                });
            }
        }
        // Combine the stacks and order them.
        {
            let mut all: Vec<(usize, usize, bool, usize)> = Vec::new();
            for (_, s) in self.delim_stacks.iter() {
                for e in s {
                    all.push((e.start, e.end, e.is_ambiguous, e.length));
                }
            }
            all.sort_by_key(|x| x.0);
            self.delim_stack = all;
        }
        // Build the HTML elements.
        if !self.regions.is_empty() {
            self.regions.sort_by_key(|r| r.a);
            let (start, end) = (self.regions[0].a, self.regions[0].d);
            let (el, count) = self.build_element(data, 0, 0);
            self.increment_next_position(start, count);
            return MatchOutcome::Accept {
                node: NewNode::El(el),
                start,
                end,
            };
        }
        // Failed to pair: avoid the parsed range next pass.
        let start = ms;
        let end = self.delim_stack.last().map(|e| e.1).unwrap_or(me);
        self.delim_reset();
        MatchOutcome::Advance(start.max(end).max(ms))
    }
}

/// Branch 2 of the pairing loop.
#[allow(clippy::too_many_arguments)]
fn handle_branch2(
    inl: &mut Inline<'_>,
    _data: &str,
    token: &str,
    start: &mut usize,
    end: &mut usize,
    current: &mut usize,
    last: &mut usize,
    is_end: &mut bool,
    is_start: &mut bool,
    is_ambiguous: bool,
) -> BranchFlow {
    let stack_empty = inl.delim_stack_of(token).is_empty();
    if !stack_empty && *is_end && (*last >= 3 || !is_ambiguous) && *last > *current {
        // Upstream clears `is_start` on taking branch 2 (the (start,end)
        // pushback below replaces the plain opener push).
        *is_start = false;
        // Pop past ambiguous non-triple openers to the real one: the
        // loop tests the POPPED delimiter, not the new top.
        let mut delimiter = inl.delim_stack_of(token).pop().unwrap();
        loop {
            if inl.delim_stack_of(token).is_empty() {
                break;
            }
            if delimiter.length == 3 || !delimiter.is_ambiguous {
                break;
            }
            delimiter = inl.delim_stack_of(token).pop().unwrap();
            *last = delimiter.length;
        }
        if delimiter.is_ambiguous && delimiter.length != 3 {
            if inl.delim_stacks.iter().any(|(t, s)| t != token && !s.is_empty()) {
                // Token-local reset only (regions/siblings kept).
                inl.delim_stack_of(token).clear();
                return BranchFlow::Continue;
            }
            return BranchFlow::Break;
        }
        *is_end = false;
        let (ds, mut de) = (delimiter.start, delimiter.end);
        while *current > 0 {
            let max_size = 2usize;
            let size = (*current).min(if *current == 3 { 1 } else { max_size });
            let new = *last - size;
            inl.add_region(token, ds + new, de, *start, *start + size, size);
            *start += size;
            *current -= size;
            *last -= size;
            de -= size;
        }
        if *last > 0 {
            inl.delim_stack_of(token).push(DelimEntry {
                start: ds,
                end: de,
                is_ambiguous: false,
                length: *last,
            });
        }
    }
    let _ = end;
    BranchFlow::Done
}

enum BranchFlow {
    Break,
    Continue,
    Done,
}

/// Trigger-byte bit assignments for the inline prefilter.
const T_BACKTICK: u16 = 1;
const T_BSLASH: u16 = 2;
const T_LBRACK: u16 = 4;
const T_BANG: u16 = 8;
const T_LT: u16 = 16;
const T_AMP: u16 = 32;
const T_STAR: u16 = 64;
const T_US: u16 = 128;
const T_NL: u16 = 256;
const T_DASH: u16 = 512;
const T_DOT: u16 = 1024;
const T_SQ: u16 = 2048;
const T_DQ: u16 = 4096;
const T_GT: u16 = 8192;

/// Required trigger bytes for a main-pass pattern (0 = none possible).
fn pat_mask(pat: Pat) -> u16 {
    match pat {
        Pat::Backtick => T_BACKTICK,
        Pat::Escape => T_BSLASH,
        Pat::Reference | Pat::ShortReference | Pat::Link | Pat::Footnote | Pat::WikiLink => T_LBRACK,
        Pat::ImageLink | Pat::ImageReference | Pat::ShortImageReference => T_LBRACK | T_BANG,
        Pat::Autolink | Pat::Automail | Pat::Html | Pat::HtmlStrict => T_LT,
        Pat::Linebreak | Pat::Nl => T_NL,
        Pat::Entity => T_AMP,
        Pat::EmStrong => T_STAR | T_US,
        Pat::Smarty(_) => u16::MAX,
    }
}

/// Byte-presence mask of trigger bytes in `data`.
fn text_mask(data: &str) -> u16 {
    let mut m = 0u16;
    for b in data.bytes() {
        match b {
            b'`' => m |= T_BACKTICK,
            b'\\' => m |= T_BSLASH,
            b'[' => m |= T_LBRACK,
            b'!' => m |= T_BANG,
            b'<' => m |= T_LT,
            b'&' => m |= T_AMP,
            b'*' => m |= T_STAR,
            b'_' => m |= T_US,
            b'\n' => m |= T_NL,
            b'-' => m |= T_DASH,
            b'.' => m |= T_DOT,
            b'\'' => m |= T_SQ,
            b'"' => m |= T_DQ,
            b'>' => m |= T_GT,
            _ => {}
        }
        if m == 0x3FFF {
            break;
        }
    }
    m
}

/// Name of the last matched named group (`ambiguous`/`end`/`start`).
fn last_group_name(caps: &Captures) -> String {
    for name in ["ambiguous", "end", "start"] {
        if caps.name(name).is_some() {
            return name.to_string();
        }
    }
    String::new()
}

/// Token char (`*`/`_`) at a trigger position.
fn trigger_token(data: &str, pos: usize) -> String {
    data[pos..].chars().next().unwrap_or('\0').to_string()
}
impl<'a> Inline<'a> {
    // ---- DelimiterProcessor cache + element builder ----

    fn increment_next_position(&mut self, start: usize, count: usize) {
        self.cache_index += count;
        if self.cache_index < self.regions.len() {
            self.cache_pos = self.regions[self.cache_index].a;
            // Upstream pops from the front until one entry lands in
            // `(start, cache_pos]` (stale entries are discarded, not kept).
            while !self.delim_stack.is_empty() {
                let e = self.delim_stack.remove(0);
                if start < e.0 && e.0 <= self.cache_pos {
                    self.cache_pos = e.0;
                    break;
                }
            }
        } else {
            self.delim_reset();
        }
    }

    fn get_cached_result(&mut self, data: &str, pos: usize) -> (Element, usize, usize) {
        let ci = self.cache_index;
        let offset: isize = if pos != self.cache_pos {
            pos as isize - self.cache_pos as isize
        } else {
            pos as isize - self.regions[ci].a as isize
        };
        let start = self.regions[ci].a;
        let end = self.regions[ci].d;
        let (el, count) = self.build_element(data, ci, offset);
        self.increment_next_position(start, count);
        (el, (start as isize + offset) as usize, (end as isize + offset) as usize)
    }

    /// `_build_element(data, start, offset)` — exact port. `last`/`outer`
    /// track nodes by index *path* from the root (pushes never invalidate
    /// paths since nothing is removed mid-build).
    fn build_element(&mut self, data: &str, start: usize, offset: isize) -> (Element, usize) {
        let regions = self.regions.clone();
        let end = regions.len();
        // Root placeholder; replaced by the first element.
        let mut el = Element::new("__root__");
        let mut outer: Vec<Vec<usize>> = Vec::new();
        let mut outer_r: Vec<Region> = Vec::new();
        let mut last: Vec<usize> = Vec::new();
        let mut previous: Option<Region> = None;
        let mut idx = 0usize;
        for i in start..end {
            idx += 1;
            let r = regions[i].clone();
            // `if idx and r[0] >= regions[start][3]` (idx >= 1 always).
            if r.a >= regions[start].d {
                idx -= 1;
                break;
            }
            let tag = if r.size == 2 {
                r.tags.0.clone()
            } else {
                r.tags.1.clone()
            };
            let el1 = Element::new(&tag);
            if idx > 1 {
                let prev = previous.clone().unwrap();
                {
                    let l = nav(&mut el, &last);
                    if l.text.is_none() {
                        let t = if (prev.c as isize) < (r.a as isize) {
                            pyslice(data, prev.b as isize + offset, prev.c as isize + offset)
                        } else {
                            pyslice(data, prev.b as isize + offset, r.a as isize + offset)
                        };
                        l.text = Some(Text::Normal(t.to_string()));
                    }
                }
                if last != *outer.last().unwrap() {
                    let or_ = outer_r.last().unwrap().clone();
                    let l = nav(&mut el, &last);
                    if l.tail.is_none() {
                        if (r.a as isize) < (or_.d as isize) {
                            l.tail = Some(Text::Normal(
                                pyslice(data, prev.d as isize + offset, r.a as isize + offset)
                                    .to_string(),
                            ));
                        } else {
                            l.tail = Some(Text::Normal(
                                pyslice(data, prev.d as isize + offset, or_.c as isize + offset)
                                    .to_string(),
                            ));
                            let o = nav(&mut el, outer.last().unwrap());
                            o.tail = Some(Text::Normal(
                                pyslice(data, or_.d as isize + offset, r.a as isize + offset)
                                    .to_string(),
                            ));
                        }
                    }
                }
            }
            if idx == 1 {
                el = el1;
                last = Vec::new();
                outer.push(Vec::new());
                outer_r.push(r.clone());
            } else {
                while outer_r.len() > 1 && r.d > outer_r.last().unwrap().d {
                    outer.pop();
                    outer_r.pop();
                }
                // `while/else`: the loop has no `break`, so the append
                // always runs.
                let oi: Vec<usize>;
                {
                    let o = nav(&mut el, outer.last().unwrap());
                    o.children.push(el1);
                    let mut pp = outer.last().unwrap().clone();
                    pp.push(o.children.len() - 1);
                    oi = pp;
                }
                last = oi;
                if i + 1 < end && r.d > regions[i + 1].d {
                    outer.push(last.clone());
                    outer_r.push(r.clone());
                }
            }
            previous = Some(r);
        }
        // Populate remaining elements with their text.
        while !outer.is_empty() {
            {
                let l = nav(&mut el, &last);
                if l.text.is_none() {
                    let prev = previous.clone().unwrap();
                    l.text = Some(Text::Normal(
                        pyslice(data, prev.b as isize + offset, prev.c as isize + offset)
                            .to_string(),
                    ));
                }
                if l.tail.is_none() && last != *outer.last().unwrap() {
                    let prev = previous.clone().unwrap();
                    if let Some(or_) = outer_r.last() {
                        l.tail = Some(Text::Normal(
                            pyslice(data, prev.d as isize + offset, or_.c as isize + offset)
                                .to_string(),
                        ));
                    }
                }
            }
            last = outer.pop().unwrap();
            previous = outer_r.pop();
        }
        (el, idx)
    }
}

/// Navigate to a node by index path from `root`.
fn nav<'x>(root: &'x mut Element, path: &[usize]) -> &'x mut Element {
    let mut node = root;
    for &i in path {
        node = &mut node.children[i];
    }
    node
}

/// Python-semantics slice: clamps, negative-from-end, char-boundary snap.
fn pyslice(data: &str, start: isize, end: isize) -> &str {
    let n = data.len() as isize;
    let mut s = if start < 0 { (n + start).max(0) } else { start.min(n) };
    let mut e = if end < 0 { (n + end).max(0) } else { end.min(n) };
    if s > e {
        return "";
    }
    while s < n && !data.is_char_boundary(s as usize) {
        s += 1;
    }
    while e > 0 && !data.is_char_boundary(e as usize) {
        e -= 1;
    }
    if s > e {
        ""
    } else {
        &data[s as usize..e as usize]
    }
}
impl<'a> Inline<'a> {
    // ---- FootnoteInlineProcessor (175) ----

    fn match_footnote(&mut self, data: &str, ms: usize, me: usize) -> MatchOutcome {
        let caps = match pat_regex(Pat::Footnote).captures_from_pos(data, ms) {
            Ok(Some(c)) if c.get(0).unwrap().start() == ms => c,
            _ => return MatchOutcome::Reject,
        };
        let id = caps.get(1).unwrap().as_str().to_string();
        if !self.ctx.footnotes.iter().any(|(k, _)| k == &id) {
            return MatchOutcome::Reject;
        }
        crate::ext::footnotes::add_footnote_ref(self.ctx, &id);
        let num = if !self.ctx.ext_bool("footnotes", "USE_DEFINITION_ORDER", true) {
            self.ctx.footnote_order.iter().position(|x| x == &id).unwrap_or(0) + 1
        } else {
            self.ctx.footnotes.iter().position(|(k, _)| k == &id).unwrap_or(0) + 1
        };
        let ref_id = match crate::ext::footnotes::make_footnote_ref_id(self.ctx, &id, true) {
            Ok(s) => s,
            Err(_) => return MatchOutcome::Reject,
        };
        let mut sup = Element::new("sup");
        let mut a = Element::new("a");
        sup.set_attr("id", &ref_id);
        a.set_attr("href", &format!("#{}", crate::ext::footnotes::make_footnote_id(self.ctx, &id)));
        a.set_attr("class", "footnote-ref");
        let tmpl = self.ctx.ext_str("footnotes", "SUPERSCRIPT_TEXT", "{}");
        a.text = Some(Text::Normal(crate::ext::footnotes::format_one(&tmpl, num)));
        sup.children.push(a);
        MatchOutcome::Accept {
            node: NewNode::El(sup),
            start: ms,
            end: me,
        }
    }
}
impl<'a> Inline<'a> {
    // ---- SubstituteTextPattern (smarty) ----

    fn match_smarty(&mut self, idx: usize, data: &str, ms: usize, me: usize) -> MatchOutcome {
        if me <= ms {
            // Zero-width matches accept nothing; accepting would grow
            // `data` by a placeholder per step without consuming input.
            return MatchOutcome::Reject;
        }
        let (re, rep): (F, Vec<crate::ext::smarty::Repl>) = {
            let p = &self.smarty[idx].1;
            (p.re.clone(), p.replace.clone())
        };
        let caps = match re.captures_from_pos(data, ms) {
            Ok(Some(c)) if c.get(0).unwrap().start() == ms => c,
            _ => return MatchOutcome::Reject,
        };
        let mut result = String::new();
        for r in &rep {
            match r {
                crate::ext::smarty::Repl::Group(g) => {
                    if let Some(m) = caps.get(*g) {
                        result.push_str(m.as_str());
                    }
                }
                crate::ext::smarty::Repl::Text(s) => {
                    let key = self.ctx.stash.len();
                    self.ctx
                        .stash
                        .push(crate::context::StashEntry::Html(s.clone()));
                    result.push_str(&crate::consts::placeholder_html(key));
                }
            }
        }
        let _ = me;
        MatchOutcome::Accept {
            node: NewNode::Str(result),
            start: ms,
            end: caps.get(0).unwrap().end(),
        }
    }

    // ---- WikiLinksInlineProcessor ----

    fn match_wikilink(&mut self, data: &str, ms: usize, me: usize) -> MatchOutcome {
        let caps = match pat_regex(Pat::WikiLink).captures_from_pos(data, ms) {
            Ok(Some(c)) if c.get(0).unwrap().start() == ms => c,
            _ => return MatchOutcome::Reject,
        };
        let label = caps.get(1).unwrap().as_str().trim().to_string();
        if label.is_empty() {
            return MatchOutcome::Accept {
                node: NewNode::Str(String::new()),
                start: ms,
                end: me,
            };
        }
        // `build_url`, default or custom (custom declines upstream).
        let is_default = matches!(
            self.ctx.ext_config("wikilinks", "build_url"),
            Some(crate::ConfigValue::Str(s)) if s == "default"
        );
        if !is_default {
            return MatchOutcome::Advance(me);
        }
        let base = self.ctx.ext_str("wikilinks", "base_url", "/");
        let end = self.ctx.ext_str("wikilinks", "end_url", "/");
        let html_class = self.ctx.ext_str("wikilinks", "html_class", "wikilink");
        // `md.Meta` overrides (only when the meta extension is present).
        let (mut base, mut end, mut html_class) = (base, end, html_class);
        if self.ctx.has_extension("meta") {
            for (k, vs) in &self.ctx.meta {
                if let Some(v) = vs.first() {
                    match k.as_str() {
                        "wiki_base_url" => base = v.clone(),
                        "wiki_end_url" => end = v.clone(),
                        "wiki_html_class" => html_class = v.clone(),
                        _ => {}
                    }
                }
            }
        }
        let url = build_wikilink_url(&label, &base, &end);
        let mut a = Element::new("a");
        a.text = Some(Text::Normal(label));
        a.set_attr("href", &url);
        if !html_class.is_empty() {
            a.set_attr("class", &html_class);
        }
        MatchOutcome::Accept {
            node: NewNode::El(a),
            start: ms,
            end: me,
        }
    }
}

/// `build_url` default: `re.sub(r'([ ]+_)|(_[ ]+)|([ ]+)', '_', label)`.
fn build_wikilink_url(label: &str, base: &str, end: &str) -> String {
    let re = compile(r"([ ]+_)|(_[ ]+)|([ ]+)");
    let clean = re.replace_all(label, "_").into_owned();
    format!("{base}{clean}{end}")
}
